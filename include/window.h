#include <stddef.h>
#include <stdint.h>

typedef struct Window {
    size_t width;
    size_t height;
    size_t x;
    size_t y;
    uint64_t shm_id;
} Window;

Window request_window(size_t width, size_t height, const char* title);

int draw_buffer_to_window(
    const uint32_t* buffer,
    uint64_t shm_id,
    size_t src_width,
    size_t src_height,
    size_t window_width,
    size_t window_height
);
