use core::sync::atomic::{Ordering};
use crate::kbrd::kbrd_server;
use crate::{halt, num_to_ascii, SCREEN, test, TextInterface};

const PROCESSES_TOTAL: usize = 1;
static mut PROCESSES: [Option<ProcessDescriptor>; PROCESSES_TOTAL+2] = [None, None, None];

static mut CURRENT_PROCESS: usize = 0;

#[derive(Copy, Clone)]
struct ProcessDescriptor{
    slices_left: u32,
    slices_max: u32,
    pid: u32,
    context: Context,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
struct Context{
    /*eflags: u32,
    cs: u32,
    eip: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
    esi: u32,
    edi: u32,
    esp: u32,
    eax: u32*/

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

extern "C"{
    fn switch_to(ctx_addr: u32);
}

pub unsafe fn init_tasks(){
    PROCESSES[2] =  Some(ProcessDescriptor{
        slices_left: 1,
        slices_max: 64,
        pid: 0,
        context: Context{
            eax: 1,
            ebx: 2,
            ecx: 3,
            edx: 4,
            esi: 5,
            edi: 6,
            esp: 0x2127000,
            ebp: 0x2127000,
            eip: 0,
            cs: 0x08,
            eflags: 0
        },
    });
    PROCESSES[1] =  Some(ProcessDescriptor{
        slices_left: 64,
        slices_max: 64,
        pid: 0,
        context: Context{
            eax: 1,
            ebx: 2,
            ecx: 3,
            edx: 4,
            esi: 5,
            edi: 6,
            esp: 0x200000,
            ebp: 0x200000,
            eip: (test as *const fn()) as u32,
            cs: 0x10,
            eflags: 0
        },
    });
    PROCESSES[0] = Some(ProcessDescriptor{
        slices_left: 64,
        slices_max: 64,
        pid: 0,
        context: Context{
            eax: 1,
            ebx: 2,
            ecx: 3,
            edx: 4,
            esi: 5,
            edi: 6,
            esp: 0x210000,
            ebp: 0x210000,
            eip: (kbrd_server as *const fn()) as u32,
            cs: 0x10,
            eflags: 0
        },
    });
}

#[no_mangle]
unsafe extern "C" fn swap(context: Context){

    if let Some(ref mut current_process) = PROCESSES[CURRENT_PROCESS]{
        current_process.context = context;
        current_process.slices_left -= 1;

        if current_process.slices_left > 0{
            switch_to((&current_process.context as *const Context) as u32);
        }

        current_process.slices_left = current_process.slices_max;

        CURRENT_PROCESS = (CURRENT_PROCESS+1)%PROCESSES_TOTAL;        

        //let w = num_to_ascii(CURRENT_PROCESS as u64);
        //SSCREEN.print_strln(&w, None);

        if let Some(ref mut next_process) = PROCESSES[CURRENT_PROCESS]{
            if next_process.context.eflags == 0{
                next_process.context.eflags = context.eflags;
            }
            switch_to((&next_process.context as *const Context) as u32);
        }
    }
}