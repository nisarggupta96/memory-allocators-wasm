// rlsf

#[global_allocator]
static A: rlsf::SmallGlobalTlsf = rlsf::SmallGlobalTlsf::new();
