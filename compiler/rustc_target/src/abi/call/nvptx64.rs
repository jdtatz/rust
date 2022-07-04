// Reference: PTX Writer's Guide to Interoperability
// https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability

use crate::abi::call::FnAbi;
use crate::abi::{HasDataLayout, TyAbiInterface};

pub fn compute_abi_info<Ty>(_fn_abi: &mut FnAbi<'_, Ty>) {}

pub fn compute_ptx_kernel_abi_info<'a, Ty, C>(_cx: &C, fn_abi: &mut FnAbi<'a, Ty>)
where
    Ty: TyAbiInterface<'a, C> + Copy,
    C: HasDataLayout,
{
    if !fn_abi.ret.layout.is_unit() && !fn_abi.ret.layout.is_never() {
        panic!("Kernels should not return anything other than () or !");
    }
}
