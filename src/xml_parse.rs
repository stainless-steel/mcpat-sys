use libc::{c_char, c_double, c_int};
use libc::c_char as c_bool;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ParseXML;

extern "C" {
    pub fn new_ParseXML() -> *mut ParseXML;
    pub fn delete_ParseXML(parsexml: *mut ParseXML);
    pub fn ParseXML_parse(parsexml: *mut ParseXML, filename: *mut c_char);
    pub fn ParseXML_sys(parsexml: *mut ParseXML) -> *mut root_system;
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct predictor_systemcore {
    pub prediction_width: c_int,
    pub prediction_scheme: [c_char; 20],
    pub predictor_size: c_int,
    pub predictor_entries: c_int,
    pub local_predictor_size: [c_int; 20],
    pub local_predictor_entries: c_int,
    pub global_predictor_entries: c_int,
    pub global_predictor_bits: c_int,
    pub chooser_predictor_entries: c_int,
    pub chooser_predictor_bits: c_int,
    pub predictor_accesses: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct itlb_systemcore {
    pub number_entries: c_int,
    pub cache_policy: c_int,
    pub total_hits: c_double,
    pub total_accesses: c_double,
    pub total_misses: c_double,
    pub conflicts: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct icache_systemcore {
    pub icache_config: [c_double; 20],
    pub buffer_sizes: [c_int; 20],
    pub cache_policy: c_int,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub read_misses: c_double,
    pub replacements: c_double,
    pub read_hits: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub miss_buffer_access: c_double,
    pub fill_buffer_accesses: c_double,
    pub prefetch_buffer_accesses: c_double,
    pub prefetch_buffer_writes: c_double,
    pub prefetch_buffer_reads: c_double,
    pub prefetch_buffer_hits: c_double,
    pub conflicts: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct dtlb_systemcore {
    pub number_entries: c_int,
    pub cache_policy: c_int,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub write_hits: c_double,
    pub read_hits: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub conflicts: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct dcache_systemcore {
    pub dcache_config: [c_double; 20],
    pub buffer_sizes: [c_int; 20],
    pub cache_policy: c_int,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub read_hits: c_double,
    pub write_hits: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub replacements: c_double,
    pub write_backs: c_double,
    pub miss_buffer_access: c_double,
    pub fill_buffer_accesses: c_double,
    pub prefetch_buffer_accesses: c_double,
    pub prefetch_buffer_writes: c_double,
    pub prefetch_buffer_reads: c_double,
    pub prefetch_buffer_hits: c_double,
    pub wbb_writes: c_double,
    pub wbb_reads: c_double,
    pub conflicts: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BTB_systemcore {
    pub BTB_config: [c_int; 20],
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub read_hits: c_double,
    pub write_hits: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub replacements: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_core {
    pub clock_rate: c_int,
    pub opt_local: c_bool,
    pub x86: c_bool,
    pub machine_bits: c_int,
    pub virtual_address_width: c_int,
    pub physical_address_width: c_int,
    pub opcode_width: c_int,
    pub micro_opcode_width: c_int,
    pub instruction_length: c_int,
    pub machine_type: c_int,
    pub internal_datapath_width: c_int,
    pub number_hardware_threads: c_int,
    pub fetch_width: c_int,
    pub number_instruction_fetch_ports: c_int,
    pub decode_width: c_int,
    pub issue_width: c_int,
    pub peak_issue_width: c_int,
    pub commit_width: c_int,
    pub pipelines_per_core: [c_int; 20],
    pub pipeline_depth: [c_int; 20],
    pub FPU: [c_char; 20],
    pub divider_multiplier: [c_char; 20],
    pub ALU_per_core: c_int,
    pub FPU_per_core: c_double,
    pub MUL_per_core: c_int,
    pub instruction_buffer_size: c_int,
    pub decoded_stream_buffer_size: c_int,
    pub instruction_window_scheme: c_int,
    pub instruction_window_size: c_int,
    pub fp_instruction_window_size: c_int,
    pub ROB_size: c_int,
    pub archi_Regs_IRF_size: c_int,
    pub archi_Regs_FRF_size: c_int,
    pub phy_Regs_IRF_size: c_int,
    pub phy_Regs_FRF_size: c_int,
    pub rename_scheme: c_int,
    pub checkpoint_depth: c_int,
    pub register_windows_size: c_int,
    pub LSU_order: [c_char; 20],
    pub store_buffer_size: c_int,
    pub load_buffer_size: c_int,
    pub memory_ports: c_int,
    pub Dcache_dual_pump: [c_char; 20],
    pub RAS_size: c_int,
    pub fp_issue_width: c_int,
    pub prediction_width: c_int,
    pub number_of_BTB: c_int,
    pub number_of_BPT: c_int,
    pub total_instructions: c_double,
    pub int_instructions: c_double,
    pub fp_instructions: c_double,
    pub branch_instructions: c_double,
    pub branch_mispredictions: c_double,
    pub committed_instructions: c_double,
    pub committed_int_instructions: c_double,
    pub committed_fp_instructions: c_double,
    pub load_instructions: c_double,
    pub store_instructions: c_double,
    pub total_cycles: c_double,
    pub idle_cycles: c_double,
    pub busy_cycles: c_double,
    pub instruction_buffer_reads: c_double,
    pub instruction_buffer_write: c_double,
    pub ROB_reads: c_double,
    pub ROB_writes: c_double,
    pub rename_accesses: c_double,
    pub fp_rename_accesses: c_double,
    pub rename_reads: c_double,
    pub rename_writes: c_double,
    pub fp_rename_reads: c_double,
    pub fp_rename_writes: c_double,
    pub inst_window_reads: c_double,
    pub inst_window_writes: c_double,
    pub inst_window_wakeup_accesses: c_double,
    pub inst_window_selections: c_double,
    pub fp_inst_window_reads: c_double,
    pub fp_inst_window_writes: c_double,
    pub fp_inst_window_wakeup_accesses: c_double,
    pub fp_inst_window_selections: c_double,
    pub archi_int_regfile_reads: c_double,
    pub archi_float_regfile_reads: c_double,
    pub phy_int_regfile_reads: c_double,
    pub phy_float_regfile_reads: c_double,
    pub phy_int_regfile_writes: c_double,
    pub phy_float_regfile_writes: c_double,
    pub archi_int_regfile_writes: c_double,
    pub archi_float_regfile_writes: c_double,
    pub int_regfile_reads: c_double,
    pub float_regfile_reads: c_double,
    pub int_regfile_writes: c_double,
    pub float_regfile_writes: c_double,
    pub windowed_reg_accesses: c_double,
    pub windowed_reg_transports: c_double,
    pub function_calls: c_double,
    pub context_switches: c_double,
    pub ialu_accesses: c_double,
    pub fpu_accesses: c_double,
    pub mul_accesses: c_double,
    pub cdb_alu_accesses: c_double,
    pub cdb_mul_accesses: c_double,
    pub cdb_fpu_accesses: c_double,
    pub load_buffer_reads: c_double,
    pub load_buffer_writes: c_double,
    pub load_buffer_cams: c_double,
    pub store_buffer_reads: c_double,
    pub store_buffer_writes: c_double,
    pub store_buffer_cams: c_double,
    pub store_buffer_forwards: c_double,
    pub main_memory_access: c_double,
    pub main_memory_read: c_double,
    pub main_memory_write: c_double,
    pub pipeline_duty_cycle: c_double,
    pub IFU_duty_cycle : c_double,
    pub BR_duty_cycle : c_double,
    pub LSU_duty_cycle : c_double,
    pub MemManU_I_duty_cycle: c_double,
    pub MemManU_D_duty_cycle : c_double,
    pub ALU_duty_cycle : c_double,
    pub MUL_duty_cycle : c_double,
    pub FPU_duty_cycle : c_double,
    pub ALU_cdb_duty_cycle : c_double,
    pub MUL_cdb_duty_cycle : c_double,
    pub FPU_cdb_duty_cycle : c_double,
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub predictor: predictor_systemcore,
    pub itlb: itlb_systemcore,
    pub icache: icache_systemcore,
    pub dtlb: dtlb_systemcore,
    pub dcache: dcache_systemcore,
    pub BTB: BTB_systemcore,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_L1Directory {
    pub Directory_type: c_int,
    pub Dir_config: [c_double; 20],
    pub buffer_sizes: [c_int; 20],
    pub clockrate: c_int,
    pub ports: [c_int; 20],
    pub device_type: c_int,
    pub cache_policy: c_int,
    pub threeD_stack: [c_char; 20],
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub conflicts: c_double,
    pub duty_cycle: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_L2Directory {
    pub Directory_type: c_int,
    pub Dir_config: [c_double; 20],
    pub buffer_sizes: [c_int; 20],
    pub clockrate: c_int,
    pub ports: [c_int; 20],
    pub device_type: c_int,
    pub cache_policy: c_int,
    pub threeD_stack: [c_char; 20],
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub conflicts: c_double,
    pub duty_cycle: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_L2 {
    pub L2_config: [c_double; 20],
    pub clockrate: c_int,
    pub ports: [c_int; 20],
    pub device_type: c_int,
    pub cache_policy: c_int,
    pub threeD_stack: [c_char; 20],
    pub buffer_sizes: [c_int; 20],
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub read_hits: c_double,
    pub write_hits: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub replacements: c_double,
    pub write_backs: c_double,
    pub miss_buffer_accesses: c_double,
    pub fill_buffer_accesses: c_double,
    pub prefetch_buffer_accesses: c_double,
    pub prefetch_buffer_writes: c_double,
    pub prefetch_buffer_reads: c_double,
    pub prefetch_buffer_hits: c_double,
    pub wbb_writes: c_double,
    pub wbb_reads: c_double,
    pub conflicts: c_double,
    pub duty_cycle: c_double,
    pub merged_dir: c_bool,
    pub homenode_read_accesses: c_double,
    pub homenode_write_accesses: c_double,
    pub homenode_read_hits: c_double,
    pub homenode_write_hits: c_double,
    pub homenode_read_misses: c_double,
    pub homenode_write_misses: c_double,
    pub dir_duty_cycle: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_L3 {
    pub L3_config: [c_double; 20],
    pub clockrate: c_int,
    pub ports: [c_int; 20],
    pub device_type: c_int,
    pub cache_policy: c_int,
    pub threeD_stack: [c_char; 20],
    pub buffer_sizes: [c_int; 20],
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub total_accesses: c_double,
    pub read_accesses: c_double,
    pub write_accesses: c_double,
    pub total_hits: c_double,
    pub total_misses: c_double,
    pub read_hits: c_double,
    pub write_hits: c_double,
    pub read_misses: c_double,
    pub write_misses: c_double,
    pub replacements: c_double,
    pub write_backs: c_double,
    pub miss_buffer_accesses: c_double,
    pub fill_buffer_accesses: c_double,
    pub prefetch_buffer_accesses: c_double,
    pub prefetch_buffer_writes: c_double,
    pub prefetch_buffer_reads: c_double,
    pub prefetch_buffer_hits: c_double,
    pub wbb_writes: c_double,
    pub wbb_reads: c_double,
    pub conflicts: c_double,
    pub duty_cycle: c_double,
    pub merged_dir: c_bool,
    pub homenode_read_accesses: c_double,
    pub homenode_write_accesses: c_double,
    pub homenode_read_hits: c_double,
    pub homenode_write_hits: c_double,
    pub homenode_read_misses: c_double,
    pub homenode_write_misses: c_double,
    pub dir_duty_cycle: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct xbar0_systemNoC {
    pub number_of_inputs_of_crossbars: c_int,
    pub number_of_outputs_of_crossbars: c_int,
    pub flit_bits: c_int,
    pub input_buffer_entries_per_port: c_int,
    pub ports_of_input_buffer: [c_int; 20],
    pub crossbar_accesses: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_NoC {
    pub clockrate: c_int,
    pub kind: c_bool, // type
    pub has_global_link: c_bool,
    pub topology: [c_char; 20],
    pub horizontal_nodes: c_int,
    pub vertical_nodes: c_int,
    pub link_throughput: c_int,
    pub link_latency: c_int,
    pub input_ports: c_int,
    pub output_ports: c_int,
    pub virtual_channel_per_port: c_int,
    pub flit_bits: c_int,
    pub input_buffer_entries_per_vc: c_int,
    pub ports_of_input_buffer: [c_int; 20],
    pub dual_pump: c_int,
    pub number_of_crossbars: c_int,
    pub crossbar_type: [c_char; 20],
    pub crosspoint_type: [c_char; 20],
    pub xbar0: xbar0_systemNoC,
    pub arbiter_type: c_int,
    pub chip_coverage: c_double,
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub total_accesses: c_double,
    pub duty_cycle: c_double,
    pub route_over_perc: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_mem {
    pub mem_tech_node: c_int,
    pub device_clock: c_int,
    pub peak_transfer_rate: c_int,
    pub internal_prefetch_of_DRAM_chip: c_int,
    pub capacity_per_channel: c_int,
    pub number_ranks: c_int,
    pub num_banks_of_DRAM_chip: c_int,
    pub Block_width_of_DRAM_chip: c_int,
    pub output_width_of_DRAM_chip: c_int,
    pub page_size_of_DRAM_chip: c_int,
    pub burstlength_of_DRAM_chip: c_int,
    pub memory_accesses: c_double,
    pub memory_reads: c_double,
    pub memory_writes: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_mc {
    pub peak_transfer_rate: c_double,
    pub number_mcs: c_int,
    pub withPHY: c_bool,
    pub kind: c_int, // type
    pub duty_cycle: c_double,
    pub total_load_perc: c_double,
    pub mc_clock: c_int,
    pub llc_line_length: c_int,
    pub memory_channels_per_mc: c_int,
    pub number_ranks: c_int,
    pub req_window_size_per_channel: c_int,
    pub IO_buffer_size_per_channel: c_int,
    pub databus_width: c_int,
    pub addressbus_width: c_int,
    pub LVDS: c_bool,
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub memory_accesses: c_double,
    pub memory_reads: c_double,
    pub memory_writes: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_niu {
    pub clockrate: c_int,
    pub number_units: c_int,
    pub kind: c_int, // type
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub duty_cycle: c_double,
    pub total_load_perc: c_double,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct system_pcie {
    pub clockrate: c_int,
    pub number_units: c_int,
    pub num_channels: c_int,
    pub kind: c_int, // type
    pub withPHY: c_bool,
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub duty_cycle: c_double,
    pub total_load_perc: c_double,
}

#[repr(C)]
pub struct root_system {
    pub number_of_cores: c_int,
    pub number_of_L1Directories: c_int,
    pub number_of_L2Directories: c_int,
    pub number_of_L2s: c_int,
    pub Private_L2: c_bool,
    pub number_of_L3s: c_int,
    pub number_of_NoCs: c_int,
    pub number_of_dir_levels: c_int,
    pub domain_size: c_int,
    pub first_level_dir: c_int,
    pub homogeneous_cores: c_int,
    pub homogeneous_L1Directories: c_int,
    pub homogeneous_L2Directories: c_int,
    pub core_tech_node: c_double,
    pub target_core_clockrate: c_int,
    pub target_chip_area: c_int,
    pub temperature: c_int,
    pub number_cache_levels: c_int,
    pub L1_property: c_int,
    pub L2_property: c_int,
    pub homogeneous_L2s: c_int,
    pub L3_property: c_int,
    pub homogeneous_L3s: c_int,
    pub homogeneous_NoCs: c_int,
    pub homogeneous_ccs: c_int,
    pub Max_area_deviation: c_int,
    pub Max_power_deviation: c_int,
    pub device_type: c_int,
    pub longer_channel_device: c_bool,
    pub power_gating: c_bool,
    pub Embedded: c_bool,
    pub opt_dynamic_power: c_bool,
    pub opt_lakage_power: c_bool,
    pub opt_clockrate: c_bool,
    pub opt_area: c_bool,
    pub interconnect_projection_type: c_int,
    pub machine_bits: c_int,
    pub virtual_address_width: c_int,
    pub physical_address_width: c_int,
    pub virtual_memory_page_size: c_int,
    pub total_cycles: c_double,
    pub vdd: c_double,
    pub power_gating_vcc: c_double,
    pub core: [system_core; 64],
    pub L1Directory: [system_L1Directory; 64],
    pub L2Directory: [system_L2Directory; 64],
    pub L2: [system_L2; 64],
    pub L3: [system_L3; 64],
    pub NoC: [system_NoC; 64],
    pub mem: system_mem,
    pub mc: system_mc,
    pub flashc: system_mc,
    pub niu: system_niu,
    pub pcie: system_pcie,
}
