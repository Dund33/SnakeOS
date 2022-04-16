typedef struct __attribute__((packed)) {
    unsigned int eax,
    unsigned int esp,
    unsigned int ebp,
    unsigned int edi,
    unsigned int esi,
    unsigned int edx,
    unsigned int ecx,
    unsigned int ebx,
    unsigned int eip,
    unsigned int cs,
    unsigned int eflags,
}Context;

typedef struct __attribute__((packed)) {
    unsigned int slices_left,
    unsigned int slices_max,
    unsigned int pid,
    unsigned int context,
}ProcessDescriptor;


ProcessDescriptor processes[]={
    P
}

volatile *int CURRENT_PROCESS_PTR;
volatile int CURRENT_PROCESS;
