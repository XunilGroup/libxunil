use crate::mem::{free, malloc, sbrk};
use crate::util::{Locked, align_up};
use core::alloc::AllocError;
use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
pub static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 128 * 1024 * 1024; // 128 MiB

pub struct LinkedNode {
    pub size: usize,
    pub next: Option<&'static mut LinkedNode>,
}

impl LinkedNode {
    pub const fn new(size: usize) -> LinkedNode {
        LinkedNode { size, next: None }
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: LinkedNode,
}

#[derive(Clone, Copy)]
pub struct Allocation {
    pub start: usize,
    pub end: usize,
}

impl LinkedListAllocator {
    pub const fn new() -> LinkedListAllocator {
        Self {
            head: LinkedNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.add_free_memory_region(heap_start, heap_size);
        }
    }

    pub unsafe fn add_free_memory_region(&mut self, start: usize, size: usize) {
        unsafe {
            assert_eq!(align_up(start, 16), start);
            assert!(size >= core::mem::size_of::<LinkedNode>());

            let mut node = LinkedNode::new(size);
            node.next = self.head.next.take();

            let node_ptr = start as *mut LinkedNode;
            node_ptr.write(node);
            self.head.next = Some(&mut *node_ptr);
        }
    }

    pub fn find_region(&mut self, size: usize) -> Option<Allocation> {
        let mut current = &mut self.head;

        while let Some(ref region) = current.next {
            let mut alloc = match Self::alloc_from_region(region, size) {
                Ok(a) => a,
                Err(()) => {
                    current = current.next.as_mut().unwrap();
                    continue;
                }
            };

            let taken = current.next.take().unwrap();
            let region_end = taken.end_addr();

            let old_next = unsafe {
                let node_ptr = taken as *mut LinkedNode;
                let next_ptr = core::ptr::addr_of_mut!((*node_ptr).next);
                core::ptr::read(next_ptr)
            };

            let excess_size = region_end - alloc.end;

            if excess_size >= core::mem::size_of::<LinkedNode>() {
                unsafe {
                    let remainder_ptr = alloc.end as *mut LinkedNode;
                    remainder_ptr.write(LinkedNode {
                        size: excess_size,
                        next: old_next,
                    });
                    current.next = Some(&mut *remainder_ptr);
                }
            } else {
                alloc.end = region_end;
                current.next = old_next;
            }

            return Some(alloc);
        }

        None
    }

    fn alloc_from_region(region: &LinkedNode, size: usize) -> Result<Allocation, ()> {
        let start = region.start_addr();
        let end_unaligned = start.checked_add(size).ok_or(())?;
        let end = align_up(end_unaligned, 16);
        if end > region.end_addr() {
            return Err(());
        }
        Ok(Allocation { start, end })
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        malloc(_layout.size() as u64)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        free(_ptr)
    }
}

pub fn init_heap() -> Result<(), AllocError> {
    let memory = sbrk(HEAP_SIZE as i64) as *const u8;

    unsafe {
        ALLOCATOR.lock().init(memory.addr(), HEAP_SIZE);
    }

    Ok(())
}
