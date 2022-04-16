use crate::kbrd::kbrd_server;
use crate::tick;

const PROCESSES_TOTAL: usize = 2;
static mut PROCESSES: [Option<ProcessDescriptor>; PROCESSES_TOTAL] = [None, None];
#[no_mangle]
static mut CURRENT_PROCESS_PTR: u32 = 0;
static mut CURRENT_PROCESS: usize = 0;

#[derive(Copy, Clone)]
struct ProcessDescriptor {
    slices_left: u32,
    slices_max: u32,
    pid: u32,
    context: Context,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Context {
    eax: u32,
    esp: u32,
    ebp: u32,
    edi: u32,
    esi: u32,
    edx: u32,
    ecx: u32,
    ebx: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
}

extern "C" {
    fn switch();
}

pub unsafe fn init_tasks() {
    PROCESSES[0] = Some(ProcessDescriptor {
        slices_left: 6,
        slices_max: 6,
        pid: 0,
        context: Context {
            eax: 1,
            ebx: 2,
            ecx: 3,
            edx: 4,
            esi: 5,
            edi: 6,
            esp: 7,
            ebp: 8,
            eip: 9,
            cs: 0x10,
            eflags: 0x2,
        },
    });
    PROCESSES[1] = Some(ProcessDescriptor {
        slices_left: 1,
        slices_max: 1,
        pid: 0,
        context: Context {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            esp: 0x510000,
            ebp: 0x510008,
            eip: (kbrd_server as *const fn()) as u32,
            cs: 0x10,
            eflags: 0x2,
        },
    });

    if let Some(ref mut process0) = PROCESSES[CURRENT_PROCESS] {
        CURRENT_PROCESS_PTR = (&process0.context as *const Context) as u32;
    }
}

#[no_mangle]
unsafe extern "C" fn swap() {
    tick();
    if let Some(ref mut current_process) = PROCESSES[CURRENT_PROCESS] {
        current_process.slices_left -= 1;

        if current_process.slices_left <= 0 {
            current_process.slices_left = current_process.slices_max;
            CURRENT_PROCESS = (CURRENT_PROCESS + 1) % PROCESSES_TOTAL;

            if let Some(ref mut next_process) = PROCESSES[CURRENT_PROCESS] {
                core::ptr::write_volatile(
                    &mut CURRENT_PROCESS_PTR as *mut u32,
                    (&next_process.context as *const Context) as u32,
                );
            }
        }
    }
    switch();
}
