/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use crate::common::HANDLE;
use std::ffi::c_void;
use windows::Win32::System::Memory::{VirtualAllocEx, VirtualFreeEx};
pub use windows::Win32::System::Memory::{
    MEM_COMMIT, MEM_DECOMMIT, MEM_FREE, MEM_LARGE_PAGES, MEM_RELEASE, MEM_REPLACE_PLACEHOLDER,
    MEM_RESERVE, MEM_RESERVE_PLACEHOLDER, MEM_RESET, MEM_RESET_UNDO, PAGE_ENCLAVE_DECOMMIT,
    PAGE_ENCLAVE_MASK, PAGE_ENCLAVE_SS_FIRST, PAGE_ENCLAVE_SS_REST, PAGE_ENCLAVE_THREAD_CONTROL,
    PAGE_ENCLAVE_UNVALIDATED, PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE,
    PAGE_EXECUTE_WRITECOPY, PAGE_GRAPHICS_COHERENT, PAGE_GRAPHICS_EXECUTE,
    PAGE_GRAPHICS_EXECUTE_READ, PAGE_GRAPHICS_EXECUTE_READWRITE, PAGE_GRAPHICS_NOACCESS,
    PAGE_GRAPHICS_NOCACHE, PAGE_GRAPHICS_READONLY, PAGE_GRAPHICS_READWRITE, PAGE_GUARD,
    PAGE_NOACCESS, PAGE_NOCACHE, PAGE_PROTECTION_FLAGS, PAGE_READONLY, PAGE_READWRITE,
    PAGE_REVERT_TO_FILE_MAP, PAGE_TARGETS_INVALID, PAGE_TARGETS_NO_UPDATE, PAGE_WRITECOMBINE,
    PAGE_WRITECOPY, SEC_64K_PAGES, SEC_COMMIT, SEC_FILE, SEC_IMAGE_NO_EXECUTE, SEC_LARGE_PAGES,
    SEC_NOCACHE, SEC_PARTITION_OWNER_HANDLE, SEC_PROTECTED_IMAGE, SEC_RESERVE, SEC_WRITECOMBINE,
    VIRTUAL_ALLOCATION_TYPE, VIRTUAL_FREE_TYPE,
};

/**
 * 保留、提交或更改指定进程的虚拟地址空间中内存区域的状态。 函数将它分配的内存初始化为零。
 * 若要指定物理内存的 NUMA 节点，请参阅 virtual_alloc_ex_numa。
 * 每个页面都有一个关联的 页面状态。 virtual_alloc_ex 函数可以执行以下操作：
 * • 提交保留页的区域
 * • 保留免费页面区域
 * • 同时保留和提交可用页面区域
 * virtual_alloc_ex 无法保留保留页。 它可以提交已提交的页面。 这意味着你可以提交一系列页面，无论它们是否已提交，并且函数不会失败。
 * 可以使用 virtual_alloc_ex 保留页块，然后对 virtual_alloc_ex 进行其他调用，以提交保留块中的单个页面。 这使进程能够保留其虚拟地址空间的范围，而无需使用物理存储，直到需要为止。
 * 如果 address 参数不为 NULL，则该函数使用 address 和 size 参数来计算要分配的页面区域。 整个页面范围的当前状态必须与 allocation_type 参数指定的分配类型兼容。 否则，函数将失败，并且不会分配任何页。 此兼容性要求不排除提交已提交的页面;请参阅前面的列表。
 * 若要执行动态生成的代码，请使用 virtual_alloc_ex 分配内存，并使用 virtual_protect_ex 函数授予 PAGE_EXECUTE 访问权限。
 * virtual_alloc_ex函数可用于在指定进程的虚拟地址空间中保留地址窗口扩展(AWE)内存区域。然后，可以使用此内存区域根据应用程序的要求将物理页映射到虚拟内存中和映射出虚拟内存。必须在allocation_type参数中设置MEM_PHYSICAL和MEM_RESERVE值。不得设置MEM_COMMIT值。
 * 页面保护必须设置为PAGE_READWRITE。virtual_free_ex 函数可以取消提交已提交页面、释放页面的存储，也可以同时取消提交和释放已提交页面。 它还可以释放保留页，使其成为免费页面。
 * 创建可执行的区域时，调用程序负责在代码设置到位后，通过适当调用 flush_instruction_cache 来确保缓存一致性。 否则，尝试在新可执行区域之外执行代码可能会产生不可预知的结果。
 * `h_process` 进程的句柄。 函数在此进程的虚拟地址空间中分配内存。句柄必须具有 PROCESS_VM_OPERATION 访问权限。 有关详细信息，请参阅 进程安全和访问权限。
 * `address` 为要分配的页面区域指定所需起始地址的指针。如果要保留内存，该函数将此地址向下舍入到分配粒度的最接近倍数。如果要提交已保留的内存，该函数会将此地址向下舍入到最近的页边界。若要确定主计算机上的页面大小和分配粒度，请使用get_system_info函数。如果 lpAddress 为NULL，则该函数确定分配区域的位置。如果此地址位于尚未通过调用 initialize_enclave 进行初始化的 enclave 内， virtual_alloc_ex 会为该地址上的 enclave 分配一个零页。 该页面必须以前未提交，并且不会使用 Intel Software Guard Extensions 编程模型的 EEXTEND 指令进行测量。如果 中的地址位于你初始化的 enclave 中，则分配操作将失败并 出现ERROR_INVALID_ADDRESS 错误。 对于不支持动态内存管理的 enclave (（即 SGX1) ）也是如此。 SGX2 enclave 将允许分配，并且页面必须在分配后被 enclave 接受。
 * `size` 要分配的内存区域的大小（以字节为单位）。如果address为NULL，则该函数会将size向上舍入到下一页边界。如果address不为NULL，则该函数将分配从address到address+size范围内包含一个或多个字节的所有页。例如，这意味着跨越页边界的2字节范围会导致函数分配这两个页面。
 * `allocation_type` 内存分配的类型。 此参数必须包含以下值之一。
 * - MEM_COMMIT 从指定保留内存页的磁盘) 上的总内存大小和分页文件 (分配内存费用。 函数还保证当调用方稍后最初访问内存时，内容将为零。 除非实际访问虚拟地址，否则不会分配实际物理页。若要在一个步骤中保留和提交页面，请使用 调用 virtual_alloc_ex MEM_COMMIT | MEM_RESERVE。除非已保留整个范围，否则尝试通过指定 MEM_COMMIT 而不 指定MEM_RESERVE 和非 NULL address 来提交特定地址范围。 生成的错误代码 ERROR_INVALID_ADDRESS。尝试提交已提交的页面不会导致函数失败。 这意味着可以提交页面，而无需首先确定每个页面的当前承诺状态。如果 address 指定 enclave 中的地址，则必须MEM_COMMIT allocation_type。
 * - MEM_RESERVE 保留进程的虚拟地址空间范围，而无需在内存或磁盘上的分页文件中分配任何实际物理存储。通过使用 MEM_COMMIT 再次调用 virtual_alloc_ex 来提交保留页。 若要在一个步骤中保留和提交页面，请使用 调用 virtual_alloc_ex MEM_COMMIT | MEM_RESERVE。其他内存分配函数（如 malloc 和 local_alloc）在释放内存之前无法使用保留内存。
 * - MEM_RESET 指示 address 和 size 指定的内存范围中的数据不再感兴趣。 不应从分页文件读取或写入页面。 但是，内存块稍后将再次使用，因此不应解除提交。 此值不能与任何其他值一起使用。使用此值并不能保证使用 MEM_RESET 操作的范围将包含零。 如果希望范围包含零，请取消提交内存，然后重新提交。使用 MEM_RESET 时， virtual_alloc_ex 函数将忽略protect 的值。 但是，仍必须将 protect 设置为有效地保护值，例如 PAGE_NOACCESS。如果使用 MEM_RESET 并且内存范围映射到文件，则 virtual_alloc_ex 将返回错误。 仅当共享视图映射到分页文件时，才可接受该视图。
 * - MEM_RESET_UNDO 应仅对之前成功应用MEM_RESET的地址范围调用MEM_RESET_UNDO。 它指示调用方对 address 和 size 指定的指定内存范围中的数据感兴趣，并尝试反转 MEM_RESET的影响。 如果该函数成功，则表示指定地址范围中的所有数据都保持不变。 如果函数失败，则地址范围中至少有一些数据已替换为零。此值不能与任何其他值一起使用。 如果 对 之前未MEM_RESET的地址范围调用 MEM_RESET_UNDO ，则行为未定义。 指定 MEM_RESET时， virtual_alloc_ex 函数将忽略 protect 的值。 但是，仍必须将 protect 设置为有效地保护值，例如 PAGE_NOACCESS。Windows Server 2008 R2、Windows 7、Windows Server 2008、Windows Vista、Windows Server 2003 和 Windows XP： 在Windows 8和Windows Server 2012之前，不支持MEM_RESET_UNDO标志。
 * 此参数还可以按指示指定以下值。
 * - MEM_LARGE_PAGES 使用 大页支持分配内存。大小和对齐方式必须是大页最小值的倍数。 若要获取此值，请使用 get_large_page_minimum 函数。如果指定此值，还必须指定 MEM_RESERVE 和 MEM_COMMIT。
 * - MEM_PHYSICAL 保留可用于映射 地址窗口扩展 (AWE) 页的地址范围。此值必须与 MEM_RESERVE 一起使用，不能与其他值一起使用。
 * - MEM_TOP_DOWN 在可能的最高地址分配内存。 这比常规分配慢，尤其是在有许多分配时。
 * `protect` 要分配的页区域的内存保护。 如果正在提交页面，则可以指定任何一个 内存保护常量。如果 address 指定 enclave 中的地址， 则 protect 不能为以下任何值：
 * • PAGE_NOACCESS
 * • PAGE_GUARD
 * • PAGE_NOCACHE
 * • PAGE_WRITECOMBINE
 * 为 enclave 分配动态内存时， protect 参数必须 PAGE_READWRITE 或 PAGE_EXECUTE_READWRITE。
 * */
pub fn virtual_alloc_ex(
    h_process: HANDLE,
    address: Option<*const c_void>,
    size: usize,
    allocation_type: VIRTUAL_ALLOCATION_TYPE,
    protect: PAGE_PROTECTION_FLAGS,
) -> *mut c_void {
    unsafe { VirtualAllocEx(h_process, address, size, allocation_type, protect) }
}

/**
 * 释放、取消提交或释放和取消提交指定进程的虚拟地址空间中的内存区域。
 * `h_process` 进程的句柄。 函数释放进程的虚拟地址空间中的内存。句柄必须具有 PROCESS_VM_OPERATION 访问权限。 有关详细信息，请参阅 进程安全和访问权限。
 * `address` 指向要释放的内存区域的起始地址的指针。如果 free_type 参数 MEM_RELEASE，则 address 必须是保留区域时 virtual_alloc_ex 函数返回的基址。
 * `size` 要释放的内存区域的大小（以字节为单位）。如果 free_type 参数 MEM_RELEASE，则 size 必须为 0 (零) 。 函数释放在 对 virtual_alloc_ex 的初始分配调用中保留的整个区域。如果 free_type MEM_DECOMMIT，则函数将取消提交包含从 address 参数到 (address+size)范围内的一个或多个字节的所有内存页。 例如，这意味着跨越页边界的 2 字节内存区域会导致两个页面都解除提交。 如果 address 是 virtual_alloc_ex 返回的基址，而 size 为 0 (零) ，则函数将取消提交 virtual_alloc_ex 分配的整个区域。 之后，整个区域将处于保留状态。
 * `free_type` 释放操作的类型。 此参数须为下列值之一。
 * - MEM_DECOMMIT 取消提交已提交页面的指定区域。 操作后，页面将处于保留状态。如果尝试取消提交未提交的页面，函数不会失败。 这意味着，无需先确定当前承诺状态，即可取消提交一系列页面。当 address 参数提供 enclave 的基址时，不支持MEM_DECOMMIT值。 对于不支持动态内存管理 (（即 SGX1) ）的 enclave 也是如此。 SGX2 enclave 允许MEM_DECOMMIT enclave 中的任意位置。
 * - MEM_RELEASE释放指定的页面区域或占位符(占位符，释放地址空间并可用于)的其他分配。在执行该操作之后，这些页面将处于可用状态。如果指定此值，size必须为0(零)，并且address必须指向保留区域时virtual_alloc函数返回的基址。如果未满足上述任一条件，该函数将失败。如果当前已提交区域中的任何页面，该函数将首先取消提交，然后释放它们。如果尝试释放处于不同状态（一些保留和一些已提交）的页面，函数不会失败。这意味着，无需首先确定当前承诺状态即可发布一系列页面。
 * 使用 MEM_RELEASE 时，此参数还可以指定以下值之一。
 * - MEM_COALESCE_PLACEHOLDERS 若要合并两个相邻占位符，请指定 MEM_RELEASE | MEM_COALESCE_PLACEHOLDERS。 合并占位符时， address 和 size 必须与要合并的占位符的总体范围完全匹配。
 * - MEM_PRESERVE_PLACEHOLDER 使用 virtual_alloc2 或virtual2alloc_from_app) 将占位符替换为专用分配后，将分配释放回占位符 (。若要将占位符拆分为两个占位符，请指定 MEM_RELEASE | MEM_PRESERVE_PLACEHOLDER。
 * */
pub fn virtual_free(
    h_process: HANDLE,
    address: *mut c_void,
    size: usize,
    free_type: VIRTUAL_FREE_TYPE,
) {
    unsafe { VirtualFreeEx(h_process, address, size, free_type) }.unwrap_or(())
}
