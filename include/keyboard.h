#include <stdint.h>

#define ESCAPE 0
#define F1 1
#define F2 2
#define F3 3
#define F4 4
#define F5 5
#define F6 6
#define F7 7
#define F8 8
#define F9 9
#define F10 10
#define F11 11
#define F12 12
#define PRINT_SCREEN 13
#define SYS_RQ 14
#define SCROLL_LOCK 15
#define PAUSE_BREAK 16
#define OEM8 17
#define KEY1 18
#define KEY2 19
#define KEY3 20
#define KEY4 21
#define KEY5 22
#define KEY6 23
#define KEY7 24
#define KEY8 25
#define KEY9 26
#define KEY0 27
#define OEM_MINUS 28
#define OEM_PLUS 29
#define BACKSPACE 30
#define INSERT 31
#define HOME 32
#define PAGE_UP 33
#define NUMPAD_LOCK 34
#define NUMPAD_DIVIDE 35
#define NUMPAD_MULTIPLY 36
#define NUMPAD_SUBTRACT 37
#define TAB 38
#define Q 39
#define W 40
#define E 41
#define R 42
#define T 43
#define Y 44
#define U 45
#define I 46
#define O 47
#define P 48
#define OEM4 49
#define OEM6 50
#define OEM5 51
#define OEM7 52
#define DELETE 53
#define END 54
#define PAGE_DOWN 55
#define NUMPAD7 56
#define NUMPAD8 57
#define NUMPAD9 58
#define NUMPAD_ADD 59
#define CAPS_LOCK 60
#define A 61
#define S 62
#define D 63
#define F 64
#define G 65
#define H 66
#define J 67
#define K 68
#define L 69
#define OEM1 70
#define OEM3 71
#define RETURN 72
#define NUMPAD4 73
#define NUMPAD5 74
#define NUMPAD6 75
#define L_SHIFT 76
#define Z 77
#define X 78
#define C 79
#define V 80
#define B 81
#define N 82
#define M 83
#define OEM_COMMA 84
#define OEM_PERIOD 85
#define OEM2 86
#define R_SHIFT 87
#define ARROW_UP 88
#define NUMPAD1 89
#define NUMPAD2 90
#define NUMPAD3 91
#define NUMPAD_ENTER 92
#define L_CONTROL 93
#define L_WIN 94
#define L_ALT 95
#define SPACEBAR 96
#define R_ALT_GR 97
#define R_WIN 98
#define APPS 99
#define R_CONTROL 100
#define ARROW_LEFT 101
#define ARROW_DOWN 102
#define ARROW_RIGHT 103
#define NUMPAD0 104
#define NUMPAD_PERIOD 105
#define OEM9 106
#define OEM10 107
#define OEM11 108
#define OEM12 109
#define OEM13 110
#define PREV_TRACK 111
#define NEXT_TRACK 112
#define MUTE 113
#define CALCULATOR 114
#define PLAY 115
#define STOP 116
#define VOLUME_DOWN 117
#define VOLUME_UP 118
#define WWW_HOME 119
#define POWER_ON_TEST_OK 120
#define TOO_MANY_KEYS 121
#define R_CONTROL2 122
#define R_ALT2 123

typedef struct __attribute__((packed)) {
    uint8_t  state;
    uint8_t  _pad1;
    uint16_t key;
    uint16_t mods;
    uint16_t _pad2;
    uint32_t unicode;
} xunil_kbd_event_t;

int kbd_read(void *buf, uint8_t n);
