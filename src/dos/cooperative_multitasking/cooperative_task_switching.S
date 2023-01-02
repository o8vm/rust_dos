# inspired from https://wiki.osdev.org/Cooperative_Multitasking

 ; .section .text
 ; .global cooperative_task_switching_assembly
cooperative_task_switching_assembly:
    pushad                      # 32 bytes
    pushfd                      # 4 bytes # pushf
    mov     eax,    cr3         #Push CR3
    push    eax

    mov     eax,    [44+esp]    #The first argument, where to save
    mov     [4+eax],        ebx
    mov     [8+eax],        ecx
    mov     [12+eax],       edx
    mov     [16+eax],       esi
    mov     [20+eax],       edi

    mov     ebx,    [36+esp]    # EAX
    mov     ecx,    [40+esp]    # IP
    mov     edx,    [20+esp]    # ESP
    add     edx,    4           # Remove the return value
    mov     esi,    [16+esp]    # EBP
    mov     edi,    [4+esp]     # EFLAGS

    mov     [eax],  ebx

    mov     [24+eax],       edx
    mov     [28+eax],       esi
    mov     [32+eax],       ecx
    mov     [36+eax],       edi
    pop     ebx                 # CR3
    mov     [40+eax],       ebx
    push    ebx                 # Goodbye again
    mov     eax,    [48+esp]    # The second argument, where to load

    mov     ebx,    [4+eax]     # EBX
    mov     ecx,    [8+eax]     # ECX
    mov     edx,    [12+eax]    # EDX
    mov     esi,    [16+eax]    # ESI
    mov     edi,    [20+eax]    # EDI
    mov     ebp,    [28+eax]    # EBP

    push    eax
    mov     eax,    [36+eax]    # EFLAGS
    push    eax
    popfd
    pop     eax

    mov     esp,    [24+eax]    # ESP ## error ?
    push    eax

    mov     eax,    [40+eax]    # CR3
    mov     cr3,    eax
    pop     eax

    push    eax
    mov     eax,    [32+eax]    # EIP
    xchg    eax,    [esp]       # Cannot use a tmp storage
    mov     eax,    [eax]       # EAX
    ret
