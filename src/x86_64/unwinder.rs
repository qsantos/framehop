use std::ops::Deref;

use super::arch::ArchX86_64;
use super::cache::CacheX86_64;
use super::unwindregs::UnwindRegsX86_64;
use crate::cache::{AllocationPolicy, MayAllocateDuringUnwind};
use crate::error::Error;
use crate::unwinder::UnwinderInternal;
use crate::unwinder::{Module, Unwinder};

pub struct UnwinderX86_64<D: Deref<Target = [u8]>, P: AllocationPolicy<D> = MayAllocateDuringUnwind>(
    UnwinderInternal<D, ArchX86_64, P>,
);

impl<D: Deref<Target = [u8]>, P: AllocationPolicy<D>> Default for UnwinderX86_64<D, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Deref<Target = [u8]>, P: AllocationPolicy<D>> UnwinderX86_64<D, P> {
    pub fn new() -> Self {
        Self(UnwinderInternal::new())
    }
}

impl<D: Deref<Target = [u8]>, P: AllocationPolicy<D>> Unwinder for UnwinderX86_64<D, P> {
    type UnwindRegs = UnwindRegsX86_64;
    type Cache = CacheX86_64<D, P>;
    type Module = Module<D>;

    fn add_module(&mut self, module: Module<D>) {
        self.0.add_module(module);
    }

    fn remove_module(&mut self, module_address_range_start: u64) {
        self.0.remove_module(module_address_range_start);
    }

    fn unwind_first<F>(
        &self,
        pc: u64,
        regs: &mut UnwindRegsX86_64,
        cache: &mut CacheX86_64<D, P>,
        read_mem: &mut F,
    ) -> Result<Option<u64>, Error>
    where
        F: FnMut(u64) -> Result<u64, ()>,
    {
        self.0.unwind_first(pc, regs, &mut cache.0, read_mem)
    }

    fn unwind_next<F>(
        &self,
        return_address: u64,
        regs: &mut UnwindRegsX86_64,
        cache: &mut CacheX86_64<D, P>,
        read_mem: &mut F,
    ) -> Result<Option<u64>, Error>
    where
        F: FnMut(u64) -> Result<u64, ()>,
    {
        self.0
            .unwind_next(return_address, regs, &mut cache.0, read_mem)
    }
}
