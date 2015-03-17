#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


#[repr(C)]
struct Struct_tm;

#[repr(C)]
struct Struct_stat;

#[repr(C)]
struct Struct_dirent;

#[repr(C)]
struct Struct_sockaddr;

#[repr(C)]
struct Struct_sockaddr_in;

#[repr(C)]
struct Struct_sockaddr_un;

#[repr(C)]
struct Struct__IO_FILE;

#[repr(C)]
struct Struct__IO_FILE_plus;

#[repr(C)]
struct ASN1_ITEM;

#[repr(C)]
struct DIR;

#[repr(C)]
struct sig_atomic_t;

#[repr(C)]
struct Struct_in6_addr;

#[repr(C)]
struct Struct_iovec;

#[repr(C)]
struct glob_t;

#[repr(C)]
struct sem_t;

#[repr(C)]
struct in_addr_t;

#[repr(C)]
struct socklen_t;

#[repr(C)]
struct in_port_t;

#[repr(C)]
struct pcre;

#[repr(C)]
struct pcre_extra;

#[repr(C)]
struct pcre_callout_block;

#[repr(C)]
struct pcre16_callout_block;

#[repr(C)]
struct pcre32_callout_block;

#[repr(C)]
struct SSL_CTX;

#[repr(C)]
struct SSL;

#[repr(C)]
struct SSL_SESSION;

#[repr(C)]
struct va_list;

#[repr(C)]
struct RSA;


pub type __u_char = ::libc::c_uchar;
pub type __u_short = ::libc::c_ushort;
pub type __u_int = ::libc::c_uint;
pub type __u_long = ::libc::c_ulong;
pub type __int8_t = ::libc::c_char;
pub type __uint8_t = ::libc::c_uchar;
pub type __int16_t = ::libc::c_short;
pub type __uint16_t = ::libc::c_ushort;
pub type __int32_t = ::libc::c_int;
pub type __uint32_t = ::libc::c_uint;
pub type __int64_t = ::libc::c_long;
pub type __uint64_t = ::libc::c_ulong;
pub type __quad_t = ::libc::c_long;
pub type __u_quad_t = ::libc::c_ulong;
pub type __dev_t = ::libc::c_ulong;
pub type __uid_t = ::libc::c_uint;
pub type __gid_t = ::libc::c_uint;
pub type __ino_t = ::libc::c_ulong;
pub type __ino64_t = ::libc::c_ulong;
pub type __mode_t = ::libc::c_uint;
pub type __nlink_t = ::libc::c_ulong;
pub type __off_t = ::libc::c_long;
pub type __off64_t = ::libc::c_long;
pub type __pid_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub __val: [::libc::c_int; 2usize],
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type __fsid_t = Struct_Unnamed1;
pub type __clock_t = ::libc::c_long;
pub type __rlim_t = ::libc::c_ulong;
pub type __rlim64_t = ::libc::c_ulong;
pub type __id_t = ::libc::c_uint;
pub type __time_t = ::libc::c_long;
pub type __useconds_t = ::libc::c_uint;
pub type __suseconds_t = ::libc::c_long;
pub type __daddr_t = ::libc::c_int;
pub type __key_t = ::libc::c_int;
pub type __clockid_t = ::libc::c_int;
pub type __timer_t = *mut ::libc::c_void;
pub type __blksize_t = ::libc::c_long;
pub type __blkcnt_t = ::libc::c_long;
pub type __blkcnt64_t = ::libc::c_long;
pub type __fsblkcnt_t = ::libc::c_ulong;
pub type __fsblkcnt64_t = ::libc::c_ulong;
pub type __fsfilcnt_t = ::libc::c_ulong;
pub type __fsfilcnt64_t = ::libc::c_ulong;
pub type __fsword_t = ::libc::c_long;
pub type __ssize_t = ::libc::c_long;
pub type __syscall_slong_t = ::libc::c_long;
pub type __syscall_ulong_t = ::libc::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::libc::c_char;
pub type __intptr_t = ::libc::c_long;
pub type __socklen_t = ::libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino64_t;
pub type ino64_t = __ino64_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off64_t;
pub type off64_t = __off64_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type useconds_t = __useconds_t;
pub type suseconds_t = __suseconds_t;
pub type size_t = ::libc::c_ulong;
pub type ulong = ::libc::c_ulong;
pub type ushort = ::libc::c_ushort;
pub type _uint = ::libc::c_uint;
pub type int8_t = ::libc::c_char;
pub type int16_t = ::libc::c_short;
pub type int32_t = ::libc::c_int;
pub type int64_t = ::libc::c_long;
pub type u_int8_t = ::libc::c_uchar;
pub type u_int16_t = ::libc::c_ushort;
pub type u_int32_t = ::libc::c_uint;
pub type u_int64_t = ::libc::c_ulong;
pub type register_t = ::libc::c_long;
pub type __sig_atomic_t = ::libc::c_int;

pub type intptr_t = __intptr_t;
pub type uintptr_t = ::libc::c_ulong;
pub type uint32_t = ::libc::c_uint;
pub type uint64_t = ::libc::c_ulong;

pub type ptrdiff_t = ::libc::c_long;



pub type ngx_int_t = intptr_t;
pub type ngx_uint_t = uintptr_t;
pub type ngx_flag_t = intptr_t;
pub type ngx_module_t = Struct_ngx_module_s;
pub type ngx_conf_t = Struct_ngx_conf_s;
pub type ngx_cycle_t = Struct_ngx_cycle_s;
pub type ngx_pool_t = Struct_ngx_pool_s;
pub type ngx_chain_t = Struct_ngx_chain_s;
pub type ngx_log_t = Struct_ngx_log_s;
pub type ngx_open_file_t = Struct_ngx_open_file_s;
pub type ngx_command_t = Struct_ngx_command_s;
pub type ngx_file_t = Struct_ngx_file_s;
pub type ngx_event_t = Struct_ngx_event_s;
pub enum Struct_ngx_event_aio_s { }
pub type ngx_event_aio_t = Struct_ngx_event_aio_s;
pub type ngx_connection_t = Struct_ngx_connection_s;
pub type ngx_event_handler_pt =
    ::std::option::Option<extern "C" fn(ev: *mut ngx_event_t) -> ()>;
pub type ngx_connection_handler_pt =
    ::std::option::Option<extern "C" fn(c: *mut ngx_connection_t) -> ()>;
pub type ngx_err_t = ::libc::c_int;
pub type ngx_atomic_int_t = ::libc::c_long;
pub type ngx_atomic_uint_t = ::libc::c_ulong;
pub type ngx_atomic_t = ngx_atomic_uint_t;
pub type ngx_rbtree_key_t = ngx_uint_t;
pub type ngx_rbtree_key_int_t = ngx_int_t;
pub type ngx_rbtree_node_t = Struct_ngx_rbtree_node_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_rbtree_node_s {
    pub key: ngx_rbtree_key_t,
    pub left: *mut ngx_rbtree_node_t,
    pub right: *mut ngx_rbtree_node_t,
    pub parent: *mut ngx_rbtree_node_t,
    pub color: u_char,
    pub data: u_char,
}
impl ::std::default::Default for Struct_ngx_rbtree_node_s {
    fn default() -> Struct_ngx_rbtree_node_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_rbtree_t = Struct_ngx_rbtree_s;
pub type ngx_rbtree_insert_pt =
    ::std::option::Option<extern "C" fn
                              (root: *mut ngx_rbtree_node_t,
                               node: *mut ngx_rbtree_node_t,
                               sentinel: *mut ngx_rbtree_node_t) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_rbtree_s {
    pub root: *mut ngx_rbtree_node_t,
    pub sentinel: *mut ngx_rbtree_node_t,
    pub insert: ngx_rbtree_insert_pt,
}
impl ::std::default::Default for Struct_ngx_rbtree_s {
    fn default() -> Struct_ngx_rbtree_s { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_msec_t = ngx_rbtree_key_t;
pub type ngx_msec_int_t = ngx_rbtree_key_int_t;
pub type ngx_tm_t = Struct_tm;
pub type ngx_socket_t = ::libc::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed71 {
    pub len: size_t,
    pub data: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed71 {
    fn default() -> Struct_Unnamed71 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_str_t = Struct_Unnamed71;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed72 {
    pub key: ngx_str_t,
    pub value: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed72 {
    fn default() -> Struct_Unnamed72 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_keyval_t = Struct_Unnamed72;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed73 {
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub data: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed73 {
    fn default() -> Struct_Unnamed73 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_variable_value_t = Struct_Unnamed73;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed74 {
    pub node: ngx_rbtree_node_t,
    pub _str: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed74 {
    fn default() -> Struct_Unnamed74 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_str_node_t = Struct_Unnamed74;
pub type ngx_fd_t = ::libc::c_int;
pub type ngx_file_info_t = Struct_stat;
pub type ngx_file_uniq_t = ino_t;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed75 {
    pub name: *mut u_char,
    pub size: size_t,
    pub addr: *mut ::libc::c_void,
    pub fd: ngx_fd_t,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed75 {
    fn default() -> Struct_Unnamed75 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_file_mapping_t = Struct_Unnamed75;
#[repr(C)]
pub struct Struct_Unnamed76 {
    pub dir: *mut DIR,
    pub de: *mut Struct_dirent,
    pub info: Struct_stat,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed76 {
    fn default() -> Struct_Unnamed76 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_dir_t = Struct_Unnamed76;
#[repr(C)]
pub struct Struct_Unnamed77 {
    pub n: size_t,
    pub pglob: glob_t,
    pub pattern: *mut u_char,
    pub log: *mut ngx_log_t,
    pub test: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed77 {
    fn default() -> Struct_Unnamed77 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_glob_t = Struct_Unnamed77;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed78 {
    pub addr: *mut u_char,
    pub size: size_t,
    pub name: ngx_str_t,
    pub log: *mut ngx_log_t,
    pub exists: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed78 {
    fn default() -> Struct_Unnamed78 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_shm_t = Struct_Unnamed78;
pub type ngx_pid_t = pid_t;
pub type ngx_spawn_proc_pt =
    ::std::option::Option<extern "C" fn
                              (cycle: *mut ngx_cycle_t,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed79 {
    pub pid: ngx_pid_t,
    pub status: ::libc::c_int,
    pub channel: [ngx_socket_t; 2usize],
    pub _proc: ngx_spawn_proc_pt,
    pub data: *mut ::libc::c_void,
    pub name: *mut ::libc::c_char,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed79 {
    fn default() -> Struct_Unnamed79 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_process_t = Struct_Unnamed79;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed80 {
    pub path: *mut ::libc::c_char,
    pub name: *mut ::libc::c_char,
    pub argv: *const *mut ::libc::c_char,
    pub envp: *const *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed80 {
    fn default() -> Struct_Unnamed80 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_exec_ctx_t = Struct_Unnamed80;
pub type ngx_uid_t = uid_t;
pub type ngx_gid_t = gid_t;
pub type ngx_log_handler_pt =
    ::std::option::Option<extern "C" fn
                              (log: *mut ngx_log_t, buf: *mut u_char,
                               len: size_t) -> *mut u_char>;
pub type ngx_log_writer_pt =
    ::std::option::Option<extern "C" fn
                              (log: *mut ngx_log_t, level: ngx_uint_t,
                               buf: *mut u_char, len: size_t) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_log_s {
    pub log_level: ngx_uint_t,
    pub file: *mut ngx_open_file_t,
    pub connection: ngx_atomic_uint_t,
    pub disk_full_time: time_t,
    pub handler: ngx_log_handler_pt,
    pub data: *mut ::libc::c_void,
    pub writer: ngx_log_writer_pt,
    pub wdata: *mut ::libc::c_void,
    pub action: *mut ::libc::c_char,
    pub next: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_ngx_log_s {
    fn default() -> Struct_ngx_log_s { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_pool_cleanup_pt =
    ::std::option::Option<extern "C" fn(data: *mut ::libc::c_void) -> ()>;
pub type ngx_pool_cleanup_t = Struct_ngx_pool_cleanup_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_pool_cleanup_s {
    pub handler: ngx_pool_cleanup_pt,
    pub data: *mut ::libc::c_void,
    pub next: *mut ngx_pool_cleanup_t,
}
impl ::std::default::Default for Struct_ngx_pool_cleanup_s {
    fn default() -> Struct_ngx_pool_cleanup_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_pool_large_t = Struct_ngx_pool_large_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_pool_large_s {
    pub next: *mut ngx_pool_large_t,
    pub alloc: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_ngx_pool_large_s {
    fn default() -> Struct_ngx_pool_large_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed81 {
    pub last: *mut u_char,
    pub end: *mut u_char,
    pub next: *mut ngx_pool_t,
    pub failed: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed81 {
    fn default() -> Struct_Unnamed81 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_pool_data_t = Struct_Unnamed81;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_pool_s {
    pub d: ngx_pool_data_t,
    pub max: size_t,
    pub current: *mut ngx_pool_t,
    pub chain: *mut ngx_chain_t,
    pub large: *mut ngx_pool_large_t,
    pub cleanup: *mut ngx_pool_cleanup_t,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_ngx_pool_s {
    fn default() -> Struct_ngx_pool_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed82 {
    pub fd: ngx_fd_t,
    pub name: *mut u_char,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed82 {
    fn default() -> Struct_Unnamed82 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_pool_cleanup_file_t = Struct_Unnamed82;
pub type ngx_buf_tag_t = *mut ::libc::c_void;
pub type ngx_buf_t = Struct_ngx_buf_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_buf_s {
    pub pos: *mut u_char,
    pub last: *mut u_char,
    pub file_pos: off_t,
    pub file_last: off_t,
    pub start: *mut u_char,
    pub end: *mut u_char,
    pub tag: ngx_buf_tag_t,
    pub file: *mut ngx_file_t,
    pub shadow: *mut ngx_buf_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub num: ::libc::c_int,
}
impl ::std::default::Default for Struct_ngx_buf_s {
    fn default() -> Struct_ngx_buf_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_chain_s {
    pub buf: *mut ngx_buf_t,
    pub next: *mut ngx_chain_t,
}
impl ::std::default::Default for Struct_ngx_chain_s {
    fn default() -> Struct_ngx_chain_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed83 {
    pub num: ngx_int_t,
    pub size: size_t,
}
impl ::std::default::Default for Struct_Unnamed83 {
    fn default() -> Struct_Unnamed83 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_bufs_t = Struct_Unnamed83;
pub type ngx_output_chain_ctx_t = Struct_ngx_output_chain_ctx_s;
pub type ngx_output_chain_filter_pt =
    ::std::option::Option<extern "C" fn
                              (ctx: *mut ::libc::c_void,
                               _in: *mut ngx_chain_t) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_output_chain_ctx_s {
    pub buf: *mut ngx_buf_t,
    pub _in: *mut ngx_chain_t,
    pub free: *mut ngx_chain_t,
    pub busy: *mut ngx_chain_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub alignment: off_t,
    pub pool: *mut ngx_pool_t,
    pub allocated: ngx_int_t,
    pub bufs: ngx_bufs_t,
    pub tag: ngx_buf_tag_t,
    pub output_filter: ngx_output_chain_filter_pt,
    pub filter_ctx: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_ngx_output_chain_ctx_s {
    fn default() -> Struct_ngx_output_chain_ctx_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed84 {
    pub out: *mut ngx_chain_t,
    pub last: *mut *mut ngx_chain_t,
    pub connection: *mut ngx_connection_t,
    pub pool: *mut ngx_pool_t,
    pub limit: off_t,
}
impl ::std::default::Default for Struct_Unnamed84 {
    fn default() -> Struct_Unnamed84 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_chain_writer_ctx_t = Struct_Unnamed84;
pub type ngx_queue_t = Struct_ngx_queue_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_queue_s {
    pub prev: *mut ngx_queue_t,
    pub next: *mut ngx_queue_t,
}
impl ::std::default::Default for Struct_ngx_queue_s {
    fn default() -> Struct_ngx_queue_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed85 {
    pub elts: *mut ::libc::c_void,
    pub nelts: ngx_uint_t,
    pub size: size_t,
    pub nalloc: ngx_uint_t,
    pub pool: *mut ngx_pool_t,
}
impl ::std::default::Default for Struct_Unnamed85 {
    fn default() -> Struct_Unnamed85 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_array_t = Struct_Unnamed85;
pub type ngx_list_part_t = Struct_ngx_list_part_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_list_part_s {
    pub elts: *mut ::libc::c_void,
    pub nelts: ngx_uint_t,
    pub next: *mut ngx_list_part_t,
}
impl ::std::default::Default for Struct_ngx_list_part_s {
    fn default() -> Struct_ngx_list_part_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed86 {
    pub last: *mut ngx_list_part_t,
    pub part: ngx_list_part_t,
    pub size: size_t,
    pub nalloc: ngx_uint_t,
    pub pool: *mut ngx_pool_t,
}
impl ::std::default::Default for Struct_Unnamed86 {
    fn default() -> Struct_Unnamed86 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_list_t = Struct_Unnamed86;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed87 {
    pub value: *mut ::libc::c_void,
    pub len: u_short,
    pub name: [u_char; 1usize],
}
impl ::std::default::Default for Struct_Unnamed87 {
    fn default() -> Struct_Unnamed87 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_elt_t = Struct_Unnamed87;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed88 {
    pub buckets: *mut *mut ngx_hash_elt_t,
    pub size: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed88 {
    fn default() -> Struct_Unnamed88 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_t = Struct_Unnamed88;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed89 {
    pub hash: ngx_hash_t,
    pub value: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed89 {
    fn default() -> Struct_Unnamed89 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_wildcard_t = Struct_Unnamed89;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed90 {
    pub key: ngx_str_t,
    pub key_hash: ngx_uint_t,
    pub value: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed90 {
    fn default() -> Struct_Unnamed90 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_key_t = Struct_Unnamed90;
pub type ngx_hash_key_pt =
    ::std::option::Option<extern "C" fn(data: *mut u_char, len: size_t)
                              -> ngx_uint_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed91 {
    pub hash: ngx_hash_t,
    pub wc_head: *mut ngx_hash_wildcard_t,
    pub wc_tail: *mut ngx_hash_wildcard_t,
}
impl ::std::default::Default for Struct_Unnamed91 {
    fn default() -> Struct_Unnamed91 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_combined_t = Struct_Unnamed91;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed92 {
    pub hash: *mut ngx_hash_t,
    pub key: ngx_hash_key_pt,
    pub max_size: ngx_uint_t,
    pub bucket_size: ngx_uint_t,
    pub name: *mut ::libc::c_char,
    pub pool: *mut ngx_pool_t,
    pub temp_pool: *mut ngx_pool_t,
}
impl ::std::default::Default for Struct_Unnamed92 {
    fn default() -> Struct_Unnamed92 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_init_t = Struct_Unnamed92;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed93 {
    pub hsize: ngx_uint_t,
    pub pool: *mut ngx_pool_t,
    pub temp_pool: *mut ngx_pool_t,
    pub keys: ngx_array_t,
    pub keys_hash: *mut ngx_array_t,
    pub dns_wc_head: ngx_array_t,
    pub dns_wc_head_hash: *mut ngx_array_t,
    pub dns_wc_tail: ngx_array_t,
    pub dns_wc_tail_hash: *mut ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed93 {
    fn default() -> Struct_Unnamed93 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_hash_keys_arrays_t = Struct_Unnamed93;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed94 {
    pub hash: ngx_uint_t,
    pub key: ngx_str_t,
    pub value: ngx_str_t,
    pub lowcase_key: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed94 {
    fn default() -> Struct_Unnamed94 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_table_elt_t = Struct_Unnamed94;
#[repr(C)]
pub struct Struct_ngx_file_s {
    pub fd: ngx_fd_t,
    pub name: ngx_str_t,
    pub info: ngx_file_info_t,
    pub offset: off_t,
    pub sys_offset: off_t,
    pub log: *mut ngx_log_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_file_s {
    fn default() -> Struct_ngx_file_s { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_path_manager_pt =
    ::std::option::Option<extern "C" fn(data: *mut ::libc::c_void) -> time_t>;
pub type ngx_path_loader_pt =
    ::std::option::Option<extern "C" fn(data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed95 {
    pub name: ngx_str_t,
    pub len: size_t,
    pub level: [size_t; 3usize],
    pub manager: ngx_path_manager_pt,
    pub loader: ngx_path_loader_pt,
    pub data: *mut ::libc::c_void,
    pub conf_file: *mut u_char,
    pub line: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed95 {
    fn default() -> Struct_Unnamed95 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_path_t = Struct_Unnamed95;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed96 {
    pub name: ngx_str_t,
    pub level: [size_t; 3usize],
}
impl ::std::default::Default for Struct_Unnamed96 {
    fn default() -> Struct_Unnamed96 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_path_init_t = Struct_Unnamed96;
#[repr(C)]
pub struct Struct_Unnamed97 {
    pub file: ngx_file_t,
    pub offset: off_t,
    pub path: *mut ngx_path_t,
    pub pool: *mut ngx_pool_t,
    pub warn: *mut ::libc::c_char,
    pub access: ngx_uint_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed97 {
    fn default() -> Struct_Unnamed97 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_temp_file_t = Struct_Unnamed97;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed98 {
    pub access: ngx_uint_t,
    pub path_access: ngx_uint_t,
    pub time: time_t,
    pub fd: ngx_fd_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed98 {
    fn default() -> Struct_Unnamed98 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_ext_rename_file_t = Struct_Unnamed98;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed99 {
    pub size: off_t,
    pub buf_size: size_t,
    pub access: ngx_uint_t,
    pub time: time_t,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed99 {
    fn default() -> Struct_Unnamed99 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_copy_file_t = Struct_Unnamed99;
pub type ngx_tree_ctx_t = Struct_ngx_tree_ctx_s;
pub type ngx_tree_init_handler_pt =
    ::std::option::Option<extern "C" fn
                              (ctx: *mut ::libc::c_void,
                               prev: *mut ::libc::c_void) -> ngx_int_t>;
pub type ngx_tree_handler_pt =
    ::std::option::Option<extern "C" fn
                              (ctx: *mut ngx_tree_ctx_t, name: *mut ngx_str_t)
                              -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_tree_ctx_s {
    pub size: off_t,
    pub fs_size: off_t,
    pub access: ngx_uint_t,
    pub mtime: time_t,
    pub init_handler: ngx_tree_init_handler_pt,
    pub file_handler: ngx_tree_handler_pt,
    pub pre_tree_handler: ngx_tree_handler_pt,
    pub post_tree_handler: ngx_tree_handler_pt,
    pub spec_handler: ngx_tree_handler_pt,
    pub data: *mut ::libc::c_void,
    pub alloc: size_t,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_ngx_tree_ctx_s {
    fn default() -> Struct_ngx_tree_ctx_s { unsafe { ::std::mem::zeroed() } }
}




#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed100 {
    pub code: *mut pcre,
    pub extra: *mut pcre_extra,
}
impl ::std::default::Default for Struct_Unnamed100 {
    fn default() -> Struct_Unnamed100 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_regex_t = Struct_Unnamed100;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed101 {
    pub pattern: ngx_str_t,
    pub pool: *mut ngx_pool_t,
    pub options: ngx_int_t,
    pub regex: *mut ngx_regex_t,
    pub captures: ::libc::c_int,
    pub named_captures: ::libc::c_int,
    pub name_size: ::libc::c_int,
    pub names: *mut u_char,
    pub err: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed101 {
    fn default() -> Struct_Unnamed101 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_regex_compile_t = Struct_Unnamed101;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed102 {
    pub regex: *mut ngx_regex_t,
    pub name: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed102 {
    fn default() -> Struct_Unnamed102 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_regex_elt_t = Struct_Unnamed102;
pub type ngx_radix_node_t = Struct_ngx_radix_node_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_radix_node_s {
    pub right: *mut ngx_radix_node_t,
    pub left: *mut ngx_radix_node_t,
    pub parent: *mut ngx_radix_node_t,
    pub value: uintptr_t,
}
impl ::std::default::Default for Struct_ngx_radix_node_s {
    fn default() -> Struct_ngx_radix_node_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed103 {
    pub root: *mut ngx_radix_node_t,
    pub pool: *mut ngx_pool_t,
    pub free: *mut ngx_radix_node_t,
    pub start: *mut ::libc::c_char,
    pub size: size_t,
}
impl ::std::default::Default for Struct_Unnamed103 {
    fn default() -> Struct_Unnamed103 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_radix_tree_t = Struct_Unnamed103;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed104 {
    pub sec: time_t,
    pub msec: ngx_uint_t,
    pub gmtoff: ngx_int_t,
}
impl ::std::default::Default for Struct_Unnamed104 {
    fn default() -> Struct_Unnamed104 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_time_t = Struct_Unnamed104;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed105 {
    pub lock: ngx_atomic_t,
    pub wait: ngx_atomic_t,
}
impl ::std::default::Default for Struct_Unnamed105 {
    fn default() -> Struct_Unnamed105 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_shmtx_sh_t = Struct_Unnamed105;
#[repr(C)]
pub struct Struct_Unnamed106 {
    pub lock: *mut ngx_atomic_t,
    pub wait: *mut ngx_atomic_t,
    pub semaphore: ngx_uint_t,
    pub sem: sem_t,
    pub spin: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed106 {
    fn default() -> Struct_Unnamed106 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_shmtx_t = Struct_Unnamed106;
pub type ngx_slab_page_t = Struct_ngx_slab_page_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_slab_page_s {
    pub slab: uintptr_t,
    pub next: *mut ngx_slab_page_t,
    pub prev: uintptr_t,
}
impl ::std::default::Default for Struct_ngx_slab_page_s {
    fn default() -> Struct_ngx_slab_page_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub struct Struct_Unnamed107 {
    pub lock: ngx_shmtx_sh_t,
    pub min_size: size_t,
    pub min_shift: size_t,
    pub pages: *mut ngx_slab_page_t,
    pub last: *mut ngx_slab_page_t,
    pub free: ngx_slab_page_t,
    pub start: *mut u_char,
    pub end: *mut u_char,
    pub mutex: ngx_shmtx_t,
    pub log_ctx: *mut u_char,
    pub zero: u_char,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub data: *mut ::libc::c_void,
    pub addr: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed107 {
    fn default() -> Struct_Unnamed107 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_slab_pool_t = Struct_Unnamed107;
#[repr(C)]
pub struct Struct_Unnamed108 {
    pub addr: in_addr_t,
    pub mask: in_addr_t,
}
impl ::std::default::Default for Struct_Unnamed108 {
    fn default() -> Struct_Unnamed108 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_in_cidr_t = Struct_Unnamed108;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed109 {
    pub family: ngx_uint_t,
    pub u: Union_Unnamed110,
}
impl ::std::default::Default for Struct_Unnamed109 {
    fn default() -> Struct_Unnamed109 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed110 {
    pub _bindgen_data_: [u32; 2usize],
}
impl Union_Unnamed110 {
    pub unsafe fn _in(&mut self) -> *mut ngx_in_cidr_t {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed110 {
    fn default() -> Union_Unnamed110 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_cidr_t = Struct_Unnamed109;
#[repr(C)]
pub struct Struct_Unnamed111 {
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub name: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed111 {
    fn default() -> Struct_Unnamed111 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_addr_t = Struct_Unnamed111;
#[repr(C)]
pub struct Struct_Unnamed112 {
    pub url: ngx_str_t,
    pub host: ngx_str_t,
    pub port_text: ngx_str_t,
    pub uri: ngx_str_t,
    pub port: in_port_t,
    pub default_port: in_port_t,
    pub family: ::libc::c_int,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub socklen: socklen_t,
    pub sockaddr: [u_char; 110usize],
    pub addrs: *mut ngx_addr_t,
    pub naddrs: ngx_uint_t,
    pub err: *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed112 {
    fn default() -> Struct_Unnamed112 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_url_t = Struct_Unnamed112;
pub type ngx_shm_zone_t = Struct_ngx_shm_zone_s;
pub type ngx_shm_zone_init_pt =
    ::std::option::Option<extern "C" fn
                              (zone: *mut ngx_shm_zone_t,
                               data: *mut ::libc::c_void) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_shm_zone_s {
    pub data: *mut ::libc::c_void,
    pub shm: ngx_shm_t,
    pub init: ngx_shm_zone_init_pt,
    pub tag: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_ngx_shm_zone_s {
    fn default() -> Struct_ngx_shm_zone_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_cycle_s {
    pub conf_ctx: *mut *mut *mut *mut ::libc::c_void,
    pub pool: *mut ngx_pool_t,
    pub log: *mut ngx_log_t,
    pub new_log: ngx_log_t,
    pub log_use_stderr: ngx_uint_t,
    pub files: *mut *mut ngx_connection_t,
    pub free_connections: *mut ngx_connection_t,
    pub free_connection_n: ngx_uint_t,
    pub reusable_connections_queue: ngx_queue_t,
    pub listening: ngx_array_t,
    pub paths: ngx_array_t,
    pub open_files: ngx_list_t,
    pub shared_memory: ngx_list_t,
    pub connection_n: ngx_uint_t,
    pub files_n: ngx_uint_t,
    pub connections: *mut ngx_connection_t,
    pub read_events: *mut ngx_event_t,
    pub write_events: *mut ngx_event_t,
    pub old_cycle: *mut ngx_cycle_t,
    pub conf_file: ngx_str_t,
    pub conf_param: ngx_str_t,
    pub conf_prefix: ngx_str_t,
    pub prefix: ngx_str_t,
    pub lock_file: ngx_str_t,
    pub hostname: ngx_str_t,
}
impl ::std::default::Default for Struct_ngx_cycle_s {
    fn default() -> Struct_ngx_cycle_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed113 {
    pub daemon: ngx_flag_t,
    pub master: ngx_flag_t,
    pub timer_resolution: ngx_msec_t,
    pub worker_processes: ngx_int_t,
    pub debug_points: ngx_int_t,
    pub rlimit_nofile: ngx_int_t,
    pub rlimit_sigpending: ngx_int_t,
    pub rlimit_core: off_t,
    pub priority: ::libc::c_int,
    pub cpu_affinity_n: ngx_uint_t,
    pub cpu_affinity: *mut uint64_t,
    pub username: *mut ::libc::c_char,
    pub user: ngx_uid_t,
    pub group: ngx_gid_t,
    pub working_directory: ngx_str_t,
    pub lock_file: ngx_str_t,
    pub pid: ngx_str_t,
    pub oldpid: ngx_str_t,
    pub env: ngx_array_t,
    pub environment: *mut *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed113 {
    fn default() -> Struct_Unnamed113 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_core_conf_t = Struct_Unnamed113;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed114 {
    pub pool: *mut ngx_pool_t,
}
impl ::std::default::Default for Struct_Unnamed114 {
    fn default() -> Struct_Unnamed114 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_core_tls_t = Struct_Unnamed114;
#[repr(C)]
pub struct Struct_Unnamed115 {
    pub connection: *mut ngx_connection_t,
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub server: ngx_str_t,
    pub log: ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed115 {
    fn default() -> Struct_Unnamed115 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_udp_connection_t = Struct_Unnamed115;
pub type ngx_resolver_ctx_t = Struct_ngx_resolver_ctx_s;
pub type ngx_resolver_handler_pt =
    ::std::option::Option<extern "C" fn(ctx: *mut ngx_resolver_ctx_t) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed116 {
    pub name: *mut u_char,
    pub queue: ngx_queue_t,
    pub ident: ngx_int_t,
    pub node: ngx_rbtree_node_t,
    pub nlen: u_short,
    pub qlen: u_short,
    pub query: *mut u_char,
    pub u: Union_Unnamed117,
    pub code: u_char,
    pub naddrs: u_short,
    pub cnlen: u_short,
    pub expire: time_t,
    pub valid: time_t,
    pub ttl: uint32_t,
    pub waiting: *mut ngx_resolver_ctx_t,
}
impl ::std::default::Default for Struct_Unnamed116 {
    fn default() -> Struct_Unnamed116 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed117 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed117 {
    pub unsafe fn addr(&mut self) -> *mut in_addr_t {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn addrs(&mut self) -> *mut *mut in_addr_t {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn cname(&mut self) -> *mut *mut u_char {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed117 {
    fn default() -> Union_Unnamed117 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_resolver_node_t = Struct_Unnamed116;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed118 {
    pub event: *mut ngx_event_t,
    pub dummy: *mut ::libc::c_void,
    pub log: *mut ngx_log_t,
    pub ident: ngx_int_t,
    pub udp_connections: ngx_array_t,
    pub last_connection: ngx_uint_t,
    pub name_rbtree: ngx_rbtree_t,
    pub name_sentinel: ngx_rbtree_node_t,
    pub addr_rbtree: ngx_rbtree_t,
    pub addr_sentinel: ngx_rbtree_node_t,
    pub name_resend_queue: ngx_queue_t,
    pub addr_resend_queue: ngx_queue_t,
    pub name_expire_queue: ngx_queue_t,
    pub addr_expire_queue: ngx_queue_t,
    pub resend_timeout: time_t,
    pub expire: time_t,
    pub valid: time_t,
    pub log_level: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed118 {
    fn default() -> Struct_Unnamed118 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_resolver_t = Struct_Unnamed118;
#[repr(C)]
pub struct Struct_ngx_resolver_ctx_s {
    pub next: *mut ngx_resolver_ctx_t,
    pub resolver: *mut ngx_resolver_t,
    pub udp_connection: *mut ngx_udp_connection_t,
    pub state: ngx_int_t,
    pub name: ngx_str_t,
    pub naddrs: ngx_uint_t,
    pub addrs: *mut ngx_addr_t,
    pub addr: ngx_addr_t,
    pub sin: Struct_sockaddr_in,
    pub handler: ngx_resolver_handler_pt,
    pub data: *mut ::libc::c_void,
    pub timeout: ngx_msec_t,
    pub quick: ngx_uint_t,
    pub recursion: ngx_uint_t,
    pub event: *mut ngx_event_t,
}
impl ::std::default::Default for Struct_ngx_resolver_ctx_s {
    fn default() -> Struct_ngx_resolver_ctx_s {
        unsafe { ::std::mem::zeroed() }
    }
}




#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed140 {
    pub ctx: *mut SSL_CTX,
    pub log: *mut ngx_log_t,
    pub buffer_size: size_t,
}
impl ::std::default::Default for Struct_Unnamed140 {
    fn default() -> Struct_Unnamed140 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_ssl_t = Struct_Unnamed140;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed141 {
    pub connection: *mut SSL,
    pub last: ngx_int_t,
    pub buf: *mut ngx_buf_t,
    pub buffer_size: size_t,
    pub handler: ngx_connection_handler_pt,
    pub saved_read_handler: ngx_event_handler_pt,
    pub saved_write_handler: ngx_event_handler_pt,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed141 {
    fn default() -> Struct_Unnamed141 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_ssl_connection_t = Struct_Unnamed141;
pub type ngx_ssl_sess_id_t = Struct_ngx_ssl_sess_id_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_ssl_sess_id_s {
    pub node: ngx_rbtree_node_t,
    pub id: *mut u_char,
    pub len: size_t,
    pub session: *mut u_char,
    pub queue: ngx_queue_t,
    pub expire: time_t,
    pub stub: *mut ::libc::c_void,
    pub sess_id: [u_char; 32usize],
}
impl ::std::default::Default for Struct_ngx_ssl_sess_id_s {
    fn default() -> Struct_ngx_ssl_sess_id_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed142 {
    pub session_rbtree: ngx_rbtree_t,
    pub sentinel: ngx_rbtree_node_t,
    pub expire_queue: ngx_queue_t,
}
impl ::std::default::Default for Struct_Unnamed142 {
    fn default() -> Struct_Unnamed142 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_ssl_session_cache_t = Struct_Unnamed142;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed143 {
    pub name: [u_char; 16usize],
    pub aes_key: [u_char; 16usize],
    pub hmac_key: [u_char; 16usize],
}
impl ::std::default::Default for Struct_Unnamed143 {
    fn default() -> Struct_Unnamed143 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_ssl_session_ticket_key_t = Struct_Unnamed143;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed144 {
    pub handler: ngx_event_handler_pt,
    pub name: *mut ::libc::c_char,
    pub delay: ngx_msec_t,
}
impl ::std::default::Default for Struct_Unnamed144 {
    fn default() -> Struct_Unnamed144 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_cache_manager_ctx_t = Struct_Unnamed144;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_command_s {
    pub name: ngx_str_t,
    pub _type: ngx_uint_t,
    pub set: ::std::option::Option<extern "C" fn
                                       (cf: *mut ngx_conf_t,
                                        cmd: *mut ngx_command_t,
                                        conf: *mut ::libc::c_void)
                                       -> *mut ::libc::c_char>,
    pub conf: ngx_uint_t,
    pub offset: ngx_uint_t,
    pub post: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_ngx_command_s {
    fn default() -> Struct_ngx_command_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_open_file_s {
    pub fd: ngx_fd_t,
    pub name: ngx_str_t,
    pub flush: ::std::option::Option<extern "C" fn
                                         (file: *mut ngx_open_file_t,
                                          log: *mut ngx_log_t) -> ()>,
    pub data: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_ngx_open_file_s {
    fn default() -> Struct_ngx_open_file_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_module_s {
    pub ctx_index: ngx_uint_t,
    pub index: ngx_uint_t,
    pub spare0: ngx_uint_t,
    pub spare1: ngx_uint_t,
    pub spare2: ngx_uint_t,
    pub spare3: ngx_uint_t,
    pub version: ngx_uint_t,
    pub ctx: *mut ::libc::c_void,
    pub commands: *mut ngx_command_t,
    pub _type: ngx_uint_t,
    pub init_master: ::std::option::Option<extern "C" fn(log: *mut ngx_log_t)
                                               -> ngx_int_t>,
    pub init_module: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> ngx_int_t>,
    pub init_process: ::std::option::Option<extern "C" fn
                                                (cycle: *mut ngx_cycle_t)
                                                -> ngx_int_t>,
    pub init_thread: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> ngx_int_t>,
    pub exit_thread: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> ()>,
    pub exit_process: ::std::option::Option<extern "C" fn
                                                (cycle: *mut ngx_cycle_t)
                                                -> ()>,
    pub exit_master: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> ()>,
    pub spare_hook0: uintptr_t,
    pub spare_hook1: uintptr_t,
    pub spare_hook2: uintptr_t,
    pub spare_hook3: uintptr_t,
    pub spare_hook4: uintptr_t,
    pub spare_hook5: uintptr_t,
    pub spare_hook6: uintptr_t,
    pub spare_hook7: uintptr_t,
}
impl ::std::default::Default for Struct_ngx_module_s {
    fn default() -> Struct_ngx_module_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed145 {
    pub name: ngx_str_t,
    pub create_conf: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> *mut ::libc::c_void>,
    pub init_conf: ::std::option::Option<extern "C" fn
                                             (cycle: *mut ngx_cycle_t,
                                              conf: *mut ::libc::c_void)
                                             -> *mut ::libc::c_char>,
}
impl ::std::default::Default for Struct_Unnamed145 {
    fn default() -> Struct_Unnamed145 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_core_module_t = Struct_Unnamed145;
#[repr(C)]
pub struct Struct_Unnamed146 {
    pub file: ngx_file_t,
    pub buffer: *mut ngx_buf_t,
    pub line: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed146 {
    fn default() -> Struct_Unnamed146 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_file_t = Struct_Unnamed146;
pub type ngx_conf_handler_pt =
    ::std::option::Option<extern "C" fn
                              (cf: *mut ngx_conf_t, dummy: *mut ngx_command_t,
                               conf: *mut ::libc::c_void)
                              -> *mut ::libc::c_char>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_conf_s {
    pub name: *mut ::libc::c_char,
    pub args: *mut ngx_array_t,
    pub cycle: *mut ngx_cycle_t,
    pub pool: *mut ngx_pool_t,
    pub temp_pool: *mut ngx_pool_t,
    pub conf_file: *mut ngx_conf_file_t,
    pub log: *mut ngx_log_t,
    pub ctx: *mut ::libc::c_void,
    pub module_type: ngx_uint_t,
    pub cmd_type: ngx_uint_t,
    pub handler: ngx_conf_handler_pt,
    pub handler_conf: *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_ngx_conf_s {
    fn default() -> Struct_ngx_conf_s { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_post_handler_pt =
    ::std::option::Option<extern "C" fn
                              (cf: *mut ngx_conf_t, data: *mut ::libc::c_void,
                               conf: *mut ::libc::c_void)
                              -> *mut ::libc::c_char>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed147 {
    pub post_handler: ngx_conf_post_handler_pt,
}
impl ::std::default::Default for Struct_Unnamed147 {
    fn default() -> Struct_Unnamed147 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_post_t = Struct_Unnamed147;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed148 {
    pub post_handler: ngx_conf_post_handler_pt,
    pub old_name: *mut ::libc::c_char,
    pub new_name: *mut ::libc::c_char,
}
impl ::std::default::Default for Struct_Unnamed148 {
    fn default() -> Struct_Unnamed148 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_deprecated_t = Struct_Unnamed148;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed149 {
    pub post_handler: ngx_conf_post_handler_pt,
    pub low: ngx_int_t,
    pub high: ngx_int_t,
}
impl ::std::default::Default for Struct_Unnamed149 {
    fn default() -> Struct_Unnamed149 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_num_bounds_t = Struct_Unnamed149;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed150 {
    pub name: ngx_str_t,
    pub value: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed150 {
    fn default() -> Struct_Unnamed150 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_enum_t = Struct_Unnamed150;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed151 {
    pub name: ngx_str_t,
    pub mask: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed151 {
    fn default() -> Struct_Unnamed151 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_conf_bitmask_t = Struct_Unnamed151;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed152 {
    pub fd: ngx_fd_t,
    pub uniq: ngx_file_uniq_t,
    pub mtime: time_t,
    pub size: off_t,
    pub fs_size: off_t,
    pub directio: off_t,
    pub read_ahead: size_t,
    pub err: ngx_err_t,
    pub failed: *mut ::libc::c_char,
    pub valid: time_t,
    pub min_uses: ngx_uint_t,
    pub disable_symlinks_from: size_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed152 {
    fn default() -> Struct_Unnamed152 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_open_file_info_t = Struct_Unnamed152;
pub type ngx_cached_open_file_t = Struct_ngx_cached_open_file_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_cached_open_file_s {
    pub node: ngx_rbtree_node_t,
    pub queue: ngx_queue_t,
    pub name: *mut u_char,
    pub created: time_t,
    pub accessed: time_t,
    pub fd: ngx_fd_t,
    pub uniq: ngx_file_uniq_t,
    pub mtime: time_t,
    pub size: off_t,
    pub err: ngx_err_t,
    pub uses: uint32_t,
    pub disable_symlinks_from: size_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub _bindgen_bitfield_2_: ::libc::c_uint,
    pub event: *mut ngx_event_t,
}
impl ::std::default::Default for Struct_ngx_cached_open_file_s {
    fn default() -> Struct_ngx_cached_open_file_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed153 {
    pub rbtree: ngx_rbtree_t,
    pub sentinel: ngx_rbtree_node_t,
    pub expire_queue: ngx_queue_t,
    pub current: ngx_uint_t,
    pub max: ngx_uint_t,
    pub inactive: time_t,
}
impl ::std::default::Default for Struct_Unnamed153 {
    fn default() -> Struct_Unnamed153 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_open_file_cache_t = Struct_Unnamed153;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed154 {
    pub cache: *mut ngx_open_file_cache_t,
    pub file: *mut ngx_cached_open_file_t,
    pub min_uses: ngx_uint_t,
    pub log: *mut ngx_log_t,
}
impl ::std::default::Default for Struct_Unnamed154 {
    fn default() -> Struct_Unnamed154 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_open_file_cache_cleanup_t = Struct_Unnamed154;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed155 {
    pub data: *mut ::libc::c_void,
    pub read: *mut ngx_event_t,
    pub write: *mut ngx_event_t,
    pub fd: ngx_fd_t,
    pub file: *mut ngx_cached_open_file_t,
    pub cache: *mut ngx_open_file_cache_t,
}
impl ::std::default::Default for Struct_Unnamed155 {
    fn default() -> Struct_Unnamed155 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_open_file_cache_event_t = Struct_Unnamed155;
pub type ngx_recv_pt =
    ::std::option::Option<extern "C" fn
                              (c: *mut ngx_connection_t, buf: *mut u_char,
                               size: size_t) -> ssize_t>;
pub type ngx_recv_chain_pt =
    ::std::option::Option<extern "C" fn
                              (c: *mut ngx_connection_t,
                               _in: *mut ngx_chain_t, limit: off_t)
                              -> ssize_t>;
pub type ngx_send_pt =
    ::std::option::Option<extern "C" fn
                              (c: *mut ngx_connection_t, buf: *mut u_char,
                               size: size_t) -> ssize_t>;
pub type ngx_send_chain_pt =
    ::std::option::Option<extern "C" fn
                              (c: *mut ngx_connection_t,
                               _in: *mut ngx_chain_t, limit: off_t)
                              -> *mut ngx_chain_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed156 {
    pub recv: ngx_recv_pt,
    pub recv_chain: ngx_recv_chain_pt,
    pub udp_recv: ngx_recv_pt,
    pub send: ngx_send_pt,
    pub send_chain: ngx_send_chain_pt,
    pub flags: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed156 {
    fn default() -> Struct_Unnamed156 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_os_io_t = Struct_Unnamed156;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed157 {
    pub iovs: *mut Struct_iovec,
    pub count: ngx_uint_t,
    pub size: size_t,
    pub nalloc: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed157 {
    fn default() -> Struct_Unnamed157 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_iovec_t = Struct_Unnamed157;
pub type ngx_listening_t = Struct_ngx_listening_s;
#[repr(C)]
pub struct Struct_ngx_listening_s {
    pub fd: ngx_socket_t,
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub addr_text_max_len: size_t,
    pub addr_text: ngx_str_t,
    pub _type: ::libc::c_int,
    pub backlog: ::libc::c_int,
    pub rcvbuf: ::libc::c_int,
    pub sndbuf: ::libc::c_int,
    pub keepidle: ::libc::c_int,
    pub keepintvl: ::libc::c_int,
    pub keepcnt: ::libc::c_int,
    pub handler: ngx_connection_handler_pt,
    pub servers: *mut ::libc::c_void,
    pub log: ngx_log_t,
    pub logp: *mut ngx_log_t,
    pub pool_size: size_t,
    pub post_accept_buffer_size: size_t,
    pub post_accept_timeout: ngx_msec_t,
    pub previous: *mut ngx_listening_t,
    pub connection: *mut ngx_connection_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub fastopen: ::libc::c_int,
}
impl ::std::default::Default for Struct_ngx_listening_s {
    fn default() -> Struct_ngx_listening_s { unsafe { ::std::mem::zeroed() } }
}
pub type Enum_Unnamed158 = ::libc::c_uint;
pub const NGX_ERROR_ALERT: ::libc::c_uint = 0;
pub const NGX_ERROR_ERR: ::libc::c_uint = 1;
pub const NGX_ERROR_INFO: ::libc::c_uint = 2;
pub const NGX_ERROR_IGNORE_ECONNRESET: ::libc::c_uint = 3;
pub const NGX_ERROR_IGNORE_EINVAL: ::libc::c_uint = 4;
pub type ngx_connection_log_error_e = Enum_Unnamed158;
pub type Enum_Unnamed159 = ::libc::c_uint;
pub const NGX_TCP_NODELAY_UNSET: ::libc::c_uint = 0;
pub const NGX_TCP_NODELAY_SET: ::libc::c_uint = 1;
pub const NGX_TCP_NODELAY_DISABLED: ::libc::c_uint = 2;
pub type ngx_connection_tcp_nodelay_e = Enum_Unnamed159;
pub type Enum_Unnamed160 = ::libc::c_uint;
pub const NGX_TCP_NOPUSH_UNSET: ::libc::c_uint = 0;
pub const NGX_TCP_NOPUSH_SET: ::libc::c_uint = 1;
pub const NGX_TCP_NOPUSH_DISABLED: ::libc::c_uint = 2;
pub type ngx_connection_tcp_nopush_e = Enum_Unnamed160;
#[repr(C)]
pub struct Struct_ngx_connection_s {
    pub data: *mut ::libc::c_void,
    pub read: *mut ngx_event_t,
    pub write: *mut ngx_event_t,
    pub fd: ngx_socket_t,
    pub recv: ngx_recv_pt,
    pub send: ngx_send_pt,
    pub recv_chain: ngx_recv_chain_pt,
    pub send_chain: ngx_send_chain_pt,
    pub listening: *mut ngx_listening_t,
    pub sent: off_t,
    pub log: *mut ngx_log_t,
    pub pool: *mut ngx_pool_t,
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub addr_text: ngx_str_t,
    pub proxy_protocol_addr: ngx_str_t,
    pub ssl: *mut ngx_ssl_connection_t,
    pub local_sockaddr: *mut Struct_sockaddr,
    pub local_socklen: socklen_t,
    pub buffer: *mut ngx_buf_t,
    pub queue: ngx_queue_t,
    pub number: ngx_atomic_uint_t,
    pub requests: ngx_uint_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_connection_s {
    fn default() -> Struct_ngx_connection_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Struct_Unnamed161 {
    pub pool: *mut ngx_pool_t,
    pub facility: ngx_uint_t,
    pub severity: ngx_uint_t,
    pub tag: ngx_str_t,
    pub server: ngx_addr_t,
    pub conn: ngx_connection_t,
    pub busy: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed161 {
    fn default() -> Struct_Unnamed161 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_syslog_peer_t = Struct_Unnamed161;
pub type ngx_http_request_t = Struct_ngx_http_request_s;
pub type ngx_http_upstream_t = Struct_ngx_http_upstream_s;
pub type ngx_http_cache_t = Struct_ngx_http_cache_s;
pub type ngx_http_file_cache_t = Struct_ngx_http_file_cache_s;
pub type ngx_http_log_ctx_t = Struct_ngx_http_log_ctx_s;
pub type ngx_http_chunked_t = Struct_ngx_http_chunked_s;
pub type ngx_http_header_handler_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               h: *mut ngx_table_elt_t, offset: ngx_uint_t)
                              -> ngx_int_t>;
pub type ngx_http_log_handler_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               sr: *mut ngx_http_request_t, buf: *mut u_char,
                               len: size_t) -> *mut u_char>;
pub type ngx_http_variable_value_t = ngx_variable_value_t;
pub type ngx_http_variable_t = Struct_ngx_http_variable_s;
pub type ngx_http_set_variable_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               v: *mut ngx_http_variable_value_t,
                               data: uintptr_t) -> ()>;
pub type ngx_http_get_variable_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               v: *mut ngx_http_variable_value_t,
                               data: uintptr_t) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_variable_s {
    pub name: ngx_str_t,
    pub set_handler: ngx_http_set_variable_pt,
    pub get_handler: ngx_http_get_variable_pt,
    pub data: uintptr_t,
    pub flags: ngx_uint_t,
    pub index: ngx_uint_t,
}
impl ::std::default::Default for Struct_ngx_http_variable_s {
    fn default() -> Struct_ngx_http_variable_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed162 {
    pub capture: ngx_uint_t,
    pub index: ngx_int_t,
}
impl ::std::default::Default for Struct_Unnamed162 {
    fn default() -> Struct_Unnamed162 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_regex_variable_t = Struct_Unnamed162;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed163 {
    pub regex: *mut ngx_regex_t,
    pub ncaptures: ngx_uint_t,
    pub variables: *mut ngx_http_regex_variable_t,
    pub nvariables: ngx_uint_t,
    pub name: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed163 {
    fn default() -> Struct_Unnamed163 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_regex_t = Struct_Unnamed163;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed164 {
    pub regex: *mut ngx_http_regex_t,
    pub value: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed164 {
    fn default() -> Struct_Unnamed164 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_map_regex_t = Struct_Unnamed164;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed165 {
    pub hash: ngx_hash_combined_t,
    pub regex: *mut ngx_http_map_regex_t,
    pub nregex: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed165 {
    fn default() -> Struct_Unnamed165 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_map_t = Struct_Unnamed165;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed166 {
    pub main_conf: *mut *mut ::libc::c_void,
    pub srv_conf: *mut *mut ::libc::c_void,
    pub loc_conf: *mut *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed166 {
    fn default() -> Struct_Unnamed166 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_conf_ctx_t = Struct_Unnamed166;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed167 {
    pub preconfiguration: ::std::option::Option<extern "C" fn
                                                    (cf: *mut ngx_conf_t)
                                                    -> ngx_int_t>,
    pub postconfiguration: ::std::option::Option<extern "C" fn
                                                     (cf: *mut ngx_conf_t)
                                                     -> ngx_int_t>,
    pub create_main_conf: ::std::option::Option<extern "C" fn
                                                    (cf: *mut ngx_conf_t)
                                                    -> *mut ::libc::c_void>,
    pub init_main_conf: ::std::option::Option<extern "C" fn
                                                  (cf: *mut ngx_conf_t,
                                                   conf: *mut ::libc::c_void)
                                                  -> *mut ::libc::c_char>,
    pub create_srv_conf: ::std::option::Option<extern "C" fn
                                                   (cf: *mut ngx_conf_t)
                                                   -> *mut ::libc::c_void>,
    pub merge_srv_conf: ::std::option::Option<extern "C" fn
                                                  (cf: *mut ngx_conf_t,
                                                   prev: *mut ::libc::c_void,
                                                   conf: *mut ::libc::c_void)
                                                  -> *mut ::libc::c_char>,
    pub create_loc_conf: ::std::option::Option<extern "C" fn
                                                   (cf: *mut ngx_conf_t)
                                                   -> *mut ::libc::c_void>,
    pub merge_loc_conf: ::std::option::Option<extern "C" fn
                                                  (cf: *mut ngx_conf_t,
                                                   prev: *mut ::libc::c_void,
                                                   conf: *mut ::libc::c_void)
                                                  -> *mut ::libc::c_char>,
}
impl ::std::default::Default for Struct_Unnamed167 {
    fn default() -> Struct_Unnamed167 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_module_t = Struct_Unnamed167;
pub type Enum_Unnamed168 = ::libc::c_uint;
pub const NGX_HTTP_INITING_REQUEST_STATE: ::libc::c_uint = 0;
pub const NGX_HTTP_READING_REQUEST_STATE: ::libc::c_uint = 1;
pub const NGX_HTTP_PROCESS_REQUEST_STATE: ::libc::c_uint = 2;
pub const NGX_HTTP_CONNECT_UPSTREAM_STATE: ::libc::c_uint = 3;
pub const NGX_HTTP_WRITING_UPSTREAM_STATE: ::libc::c_uint = 4;
pub const NGX_HTTP_READING_UPSTREAM_STATE: ::libc::c_uint = 5;
pub const NGX_HTTP_WRITING_REQUEST_STATE: ::libc::c_uint = 6;
pub const NGX_HTTP_LINGERING_CLOSE_STATE: ::libc::c_uint = 7;
pub const NGX_HTTP_KEEPALIVE_STATE: ::libc::c_uint = 8;
pub type ngx_http_state_e = Enum_Unnamed168;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed169 {
    pub name: ngx_str_t,
    pub offset: ngx_uint_t,
    pub handler: ngx_http_header_handler_pt,
}
impl ::std::default::Default for Struct_Unnamed169 {
    fn default() -> Struct_Unnamed169 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_header_t = Struct_Unnamed169;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed170 {
    pub name: ngx_str_t,
    pub offset: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed170 {
    fn default() -> Struct_Unnamed170 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_header_out_t = Struct_Unnamed170;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed171 {
    pub headers: ngx_list_t,
    pub host: *mut ngx_table_elt_t,
    pub connection: *mut ngx_table_elt_t,
    pub if_modified_since: *mut ngx_table_elt_t,
    pub if_unmodified_since: *mut ngx_table_elt_t,
    pub if_match: *mut ngx_table_elt_t,
    pub if_none_match: *mut ngx_table_elt_t,
    pub user_agent: *mut ngx_table_elt_t,
    pub referer: *mut ngx_table_elt_t,
    pub content_length: *mut ngx_table_elt_t,
    pub content_type: *mut ngx_table_elt_t,
    pub range: *mut ngx_table_elt_t,
    pub if_range: *mut ngx_table_elt_t,
    pub transfer_encoding: *mut ngx_table_elt_t,
    pub expect: *mut ngx_table_elt_t,
    pub upgrade: *mut ngx_table_elt_t,
    pub accept_encoding: *mut ngx_table_elt_t,
    pub via: *mut ngx_table_elt_t,
    pub authorization: *mut ngx_table_elt_t,
    pub keep_alive: *mut ngx_table_elt_t,
    pub x_forwarded_for: ngx_array_t,
    pub user: ngx_str_t,
    pub passwd: ngx_str_t,
    pub cookies: ngx_array_t,
    pub server: ngx_str_t,
    pub content_length_n: off_t,
    pub keep_alive_n: time_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed171 {
    fn default() -> Struct_Unnamed171 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_headers_in_t = Struct_Unnamed171;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed172 {
    pub headers: ngx_list_t,
    pub status: ngx_uint_t,
    pub status_line: ngx_str_t,
    pub server: *mut ngx_table_elt_t,
    pub date: *mut ngx_table_elt_t,
    pub content_length: *mut ngx_table_elt_t,
    pub content_encoding: *mut ngx_table_elt_t,
    pub location: *mut ngx_table_elt_t,
    pub refresh: *mut ngx_table_elt_t,
    pub last_modified: *mut ngx_table_elt_t,
    pub content_range: *mut ngx_table_elt_t,
    pub accept_ranges: *mut ngx_table_elt_t,
    pub www_authenticate: *mut ngx_table_elt_t,
    pub expires: *mut ngx_table_elt_t,
    pub etag: *mut ngx_table_elt_t,
    pub override_charset: *mut ngx_str_t,
    pub content_type_len: size_t,
    pub content_type: ngx_str_t,
    pub charset: ngx_str_t,
    pub content_type_lowcase: *mut u_char,
    pub content_type_hash: ngx_uint_t,
    pub cache_control: ngx_array_t,
    pub content_length_n: off_t,
    pub date_time: time_t,
    pub last_modified_time: time_t,
}
impl ::std::default::Default for Struct_Unnamed172 {
    fn default() -> Struct_Unnamed172 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_headers_out_t = Struct_Unnamed172;
pub type ngx_http_client_body_handler_pt =
    ::std::option::Option<extern "C" fn(r: *mut ngx_http_request_t) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed173 {
    pub temp_file: *mut ngx_temp_file_t,
    pub bufs: *mut ngx_chain_t,
    pub buf: *mut ngx_buf_t,
    pub rest: off_t,
    pub free: *mut ngx_chain_t,
    pub busy: *mut ngx_chain_t,
    pub chunked: *mut ngx_http_chunked_t,
    pub post_handler: ngx_http_client_body_handler_pt,
}
impl ::std::default::Default for Struct_Unnamed173 {
    fn default() -> Struct_Unnamed173 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_request_body_t = Struct_Unnamed173;
pub type ngx_http_addr_conf_t = Struct_ngx_http_addr_conf_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed174 {
    pub addr_conf: *mut ngx_http_addr_conf_t,
    pub conf_ctx: *mut ngx_http_conf_ctx_t,
    pub ssl_servername: *mut ngx_str_t,
    pub ssl_servername_regex: *mut ngx_http_regex_t,
    pub busy: *mut *mut ngx_buf_t,
    pub nbusy: ngx_int_t,
    pub free: *mut *mut ngx_buf_t,
    pub nfree: ngx_int_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed174 {
    fn default() -> Struct_Unnamed174 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_connection_t = Struct_Unnamed174;
pub type ngx_http_cleanup_pt =
    ::std::option::Option<extern "C" fn(data: *mut ::libc::c_void) -> ()>;
pub type ngx_http_cleanup_t = Struct_ngx_http_cleanup_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_cleanup_s {
    pub handler: ngx_http_cleanup_pt,
    pub data: *mut ::libc::c_void,
    pub next: *mut ngx_http_cleanup_t,
}
impl ::std::default::Default for Struct_ngx_http_cleanup_s {
    fn default() -> Struct_ngx_http_cleanup_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_http_post_subrequest_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               data: *mut ::libc::c_void, rc: ngx_int_t)
                              -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed175 {
    pub handler: ngx_http_post_subrequest_pt,
    pub data: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed175 {
    fn default() -> Struct_Unnamed175 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_post_subrequest_t = Struct_Unnamed175;
pub type ngx_http_postponed_request_t = Struct_ngx_http_postponed_request_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_postponed_request_s {
    pub request: *mut ngx_http_request_t,
    pub out: *mut ngx_chain_t,
    pub next: *mut ngx_http_postponed_request_t,
}
impl ::std::default::Default for Struct_ngx_http_postponed_request_s {
    fn default() -> Struct_ngx_http_postponed_request_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_http_posted_request_t = Struct_ngx_http_posted_request_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_posted_request_s {
    pub request: *mut ngx_http_request_t,
    pub next: *mut ngx_http_posted_request_t,
}
impl ::std::default::Default for Struct_ngx_http_posted_request_s {
    fn default() -> Struct_ngx_http_posted_request_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_http_handler_pt =
    ::std::option::Option<extern "C" fn(r: *mut ngx_http_request_t)
                              -> ngx_int_t>;
pub type ngx_http_event_handler_pt =
    ::std::option::Option<extern "C" fn(r: *mut ngx_http_request_t) -> ()>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_request_s {
    pub signature: uint32_t,
    pub connection: *mut ngx_connection_t,
    pub ctx: *mut *mut ::libc::c_void,
    pub main_conf: *mut *mut ::libc::c_void,
    pub srv_conf: *mut *mut ::libc::c_void,
    pub loc_conf: *mut *mut ::libc::c_void,
    pub read_event_handler: ngx_http_event_handler_pt,
    pub write_event_handler: ngx_http_event_handler_pt,
    pub cache: *mut ngx_http_cache_t,
    pub upstream: *mut ngx_http_upstream_t,
    pub upstream_states: *mut ngx_array_t,
    pub pool: *mut ngx_pool_t,
    pub header_in: *mut ngx_buf_t,
    pub headers_in: ngx_http_headers_in_t,
    pub headers_out: ngx_http_headers_out_t,
    pub request_body: *mut ngx_http_request_body_t,
    pub lingering_time: time_t,
    pub start_sec: time_t,
    pub start_msec: ngx_msec_t,
    pub method: ngx_uint_t,
    pub http_version: ngx_uint_t,
    pub request_line: ngx_str_t,
    pub uri: ngx_str_t,
    pub args: ngx_str_t,
    pub exten: ngx_str_t,
    pub unparsed_uri: ngx_str_t,
    pub method_name: ngx_str_t,
    pub http_protocol: ngx_str_t,
    pub out: *mut ngx_chain_t,
    pub main: *mut ngx_http_request_t,
    pub parent: *mut ngx_http_request_t,
    pub postponed: *mut ngx_http_postponed_request_t,
    pub post_subrequest: *mut ngx_http_post_subrequest_t,
    pub posted_requests: *mut ngx_http_posted_request_t,
    pub phase_handler: ngx_int_t,
    pub content_handler: ngx_http_handler_pt,
    pub access_code: ngx_uint_t,
    pub variables: *mut ngx_http_variable_value_t,
    pub ncaptures: ngx_uint_t,
    pub captures: *mut ::libc::c_int,
    pub captures_data: *mut u_char,
    pub limit_rate: size_t,
    pub limit_rate_after: size_t,
    pub header_size: size_t,
    pub request_length: off_t,
    pub err_status: ngx_uint_t,
    pub http_connection: *mut ngx_http_connection_t,
    pub log_handler: ngx_http_log_handler_pt,
    pub cleanup: *mut ngx_http_cleanup_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub _bindgen_bitfield_2_: ::libc::c_uint,
    pub _bindgen_bitfield_3_: ::libc::c_uint,
    pub state: ngx_uint_t,
    pub header_hash: ngx_uint_t,
    pub lowcase_index: ngx_uint_t,
    pub lowcase_header: [u_char; 32usize],
    pub header_name_start: *mut u_char,
    pub header_name_end: *mut u_char,
    pub header_start: *mut u_char,
    pub header_end: *mut u_char,
    pub uri_start: *mut u_char,
    pub uri_end: *mut u_char,
    pub uri_ext: *mut u_char,
    pub args_start: *mut u_char,
    pub request_start: *mut u_char,
    pub request_end: *mut u_char,
    pub method_end: *mut u_char,
    pub schema_start: *mut u_char,
    pub schema_end: *mut u_char,
    pub host_start: *mut u_char,
    pub host_end: *mut u_char,
    pub port_start: *mut u_char,
    pub port_end: *mut u_char,
    pub _bindgen_bitfield_4_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_http_request_s {
    fn default() -> Struct_ngx_http_request_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed176 {
    pub terminal_posted_request: ngx_http_posted_request_t,
}
impl ::std::default::Default for Struct_Unnamed176 {
    fn default() -> Struct_Unnamed176 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ephemeral_t = Struct_Unnamed176;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed177 {
    pub ip: *mut u_char,
    pub pos: *mut u_char,
    pub sp: *mut ngx_http_variable_value_t,
    pub buf: ngx_str_t,
    pub line: ngx_str_t,
    pub args: *mut u_char,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub status: ngx_int_t,
    pub request: *mut ngx_http_request_t,
}
impl ::std::default::Default for Struct_Unnamed177 {
    fn default() -> Struct_Unnamed177 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_engine_t = Struct_Unnamed177;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed178 {
    pub cf: *mut ngx_conf_t,
    pub source: *mut ngx_str_t,
    pub flushes: *mut *mut ngx_array_t,
    pub lengths: *mut *mut ngx_array_t,
    pub values: *mut *mut ngx_array_t,
    pub variables: ngx_uint_t,
    pub ncaptures: ngx_uint_t,
    pub captures_mask: ngx_uint_t,
    pub size: ngx_uint_t,
    pub main: *mut ::libc::c_void,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed178 {
    fn default() -> Struct_Unnamed178 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_compile_t = Struct_Unnamed178;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed179 {
    pub value: ngx_str_t,
    pub flushes: *mut ngx_uint_t,
    pub lengths: *mut ::libc::c_void,
    pub values: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed179 {
    fn default() -> Struct_Unnamed179 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_complex_value_t = Struct_Unnamed179;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed180 {
    pub cf: *mut ngx_conf_t,
    pub value: *mut ngx_str_t,
    pub complex_value: *mut ngx_http_complex_value_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed180 {
    fn default() -> Struct_Unnamed180 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_compile_complex_value_t = Struct_Unnamed180;
pub type ngx_http_script_code_pt =
    ::std::option::Option<extern "C" fn(e: *mut ngx_http_script_engine_t)
                              -> ()>;
pub type ngx_http_script_len_code_pt =
    ::std::option::Option<extern "C" fn(e: *mut ngx_http_script_engine_t)
                              -> size_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed181 {
    pub code: ngx_http_script_code_pt,
    pub len: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed181 {
    fn default() -> Struct_Unnamed181 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_copy_code_t = Struct_Unnamed181;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed182 {
    pub code: ngx_http_script_code_pt,
    pub index: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed182 {
    fn default() -> Struct_Unnamed182 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_var_code_t = Struct_Unnamed182;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed183 {
    pub code: ngx_http_script_code_pt,
    pub handler: ngx_http_set_variable_pt,
    pub data: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed183 {
    fn default() -> Struct_Unnamed183 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_var_handler_code_t = Struct_Unnamed183;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed184 {
    pub code: ngx_http_script_code_pt,
    pub n: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed184 {
    fn default() -> Struct_Unnamed184 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_copy_capture_code_t = Struct_Unnamed184;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed185 {
    pub code: ngx_http_script_code_pt,
    pub regex: *mut ngx_http_regex_t,
    pub lengths: *mut ngx_array_t,
    pub size: uintptr_t,
    pub status: uintptr_t,
    pub next: uintptr_t,
    pub _bindgen_bitfield_1_: uintptr_t,
    pub _bindgen_bitfield_2_: uintptr_t,
    pub _bindgen_bitfield_3_: uintptr_t,
    pub _bindgen_bitfield_4_: uintptr_t,
    pub _bindgen_bitfield_5_: uintptr_t,
    pub _bindgen_bitfield_6_: uintptr_t,
    pub _bindgen_bitfield_7_: uintptr_t,
    pub name: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed185 {
    fn default() -> Struct_Unnamed185 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_regex_code_t = Struct_Unnamed185;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed186 {
    pub code: ngx_http_script_code_pt,
    pub _bindgen_bitfield_1_: uintptr_t,
    pub _bindgen_bitfield_2_: uintptr_t,
    pub _bindgen_bitfield_3_: uintptr_t,
    pub _bindgen_bitfield_4_: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed186 {
    fn default() -> Struct_Unnamed186 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_regex_end_code_t = Struct_Unnamed186;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed187 {
    pub code: ngx_http_script_code_pt,
    pub conf_prefix: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed187 {
    fn default() -> Struct_Unnamed187 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_full_name_code_t = Struct_Unnamed187;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed188 {
    pub code: ngx_http_script_code_pt,
    pub status: uintptr_t,
    pub text: ngx_http_complex_value_t,
}
impl ::std::default::Default for Struct_Unnamed188 {
    fn default() -> Struct_Unnamed188 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_return_code_t = Struct_Unnamed188;
pub type Enum_Unnamed189 = ::libc::c_uint;
pub const ngx_http_script_file_plain: ::libc::c_uint = 0;
pub const ngx_http_script_file_not_plain: ::libc::c_uint = 1;
pub const ngx_http_script_file_dir: ::libc::c_uint = 2;
pub const ngx_http_script_file_not_dir: ::libc::c_uint = 3;
pub const ngx_http_script_file_exists: ::libc::c_uint = 4;
pub const ngx_http_script_file_not_exists: ::libc::c_uint = 5;
pub const ngx_http_script_file_exec: ::libc::c_uint = 6;
pub const ngx_http_script_file_not_exec: ::libc::c_uint = 7;
pub type ngx_http_script_file_op_e = Enum_Unnamed189;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed190 {
    pub code: ngx_http_script_code_pt,
    pub op: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed190 {
    fn default() -> Struct_Unnamed190 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_file_code_t = Struct_Unnamed190;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed191 {
    pub code: ngx_http_script_code_pt,
    pub next: uintptr_t,
    pub loc_conf: *mut *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed191 {
    fn default() -> Struct_Unnamed191 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_if_code_t = Struct_Unnamed191;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed192 {
    pub code: ngx_http_script_code_pt,
    pub lengths: *mut ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed192 {
    fn default() -> Struct_Unnamed192 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_complex_value_code_t = Struct_Unnamed192;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed193 {
    pub code: ngx_http_script_code_pt,
    pub value: uintptr_t,
    pub text_len: uintptr_t,
    pub text_data: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed193 {
    fn default() -> Struct_Unnamed193 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_script_value_code_t = Struct_Unnamed193;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed194 {
    pub lock: ngx_uint_t,
    pub events: *mut ngx_event_t,
    pub last: *mut ngx_event_t,
}
impl ::std::default::Default for Struct_Unnamed194 {
    fn default() -> Struct_Unnamed194 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_event_mutex_t = Struct_Unnamed194;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_event_s {
    pub data: *mut ::libc::c_void,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub handler: ngx_event_handler_pt,
    pub index: ngx_uint_t,
    pub log: *mut ngx_log_t,
    pub timer: ngx_rbtree_node_t,
    pub queue: ngx_queue_t,
    pub _bindgen_bitfield_2_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_event_s {
    fn default() -> Struct_ngx_event_s { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed195 {
    pub add: ::std::option::Option<extern "C" fn
                                       (ev: *mut ngx_event_t,
                                        event: ngx_int_t, flags: ngx_uint_t)
                                       -> ngx_int_t>,
    pub del: ::std::option::Option<extern "C" fn
                                       (ev: *mut ngx_event_t,
                                        event: ngx_int_t, flags: ngx_uint_t)
                                       -> ngx_int_t>,
    pub enable: ::std::option::Option<extern "C" fn
                                          (ev: *mut ngx_event_t,
                                           event: ngx_int_t,
                                           flags: ngx_uint_t) -> ngx_int_t>,
    pub disable: ::std::option::Option<extern "C" fn
                                           (ev: *mut ngx_event_t,
                                            event: ngx_int_t,
                                            flags: ngx_uint_t) -> ngx_int_t>,
    pub add_conn: ::std::option::Option<extern "C" fn
                                            (c: *mut ngx_connection_t)
                                            -> ngx_int_t>,
    pub del_conn: ::std::option::Option<extern "C" fn
                                            (c: *mut ngx_connection_t,
                                             flags: ngx_uint_t) -> ngx_int_t>,
    pub process_changes: ::std::option::Option<extern "C" fn
                                                   (cycle: *mut ngx_cycle_t,
                                                    nowait: ngx_uint_t)
                                                   -> ngx_int_t>,
    pub process_events: ::std::option::Option<extern "C" fn
                                                  (cycle: *mut ngx_cycle_t,
                                                   timer: ngx_msec_t,
                                                   flags: ngx_uint_t)
                                                  -> ngx_int_t>,
    pub init: ::std::option::Option<extern "C" fn
                                        (cycle: *mut ngx_cycle_t,
                                         timer: ngx_msec_t) -> ngx_int_t>,
    pub done: ::std::option::Option<extern "C" fn(cycle: *mut ngx_cycle_t)
                                        -> ()>,
}
impl ::std::default::Default for Struct_Unnamed195 {
    fn default() -> Struct_Unnamed195 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_event_actions_t = Struct_Unnamed195;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed196 {
    pub connections: ngx_uint_t,
    pub _use: ngx_uint_t,
    pub multi_accept: ngx_flag_t,
    pub accept_mutex: ngx_flag_t,
    pub accept_mutex_delay: ngx_msec_t,
    pub name: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed196 {
    fn default() -> Struct_Unnamed196 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_event_conf_t = Struct_Unnamed196;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed197 {
    pub name: *mut ngx_str_t,
    pub create_conf: ::std::option::Option<extern "C" fn
                                               (cycle: *mut ngx_cycle_t)
                                               -> *mut ::libc::c_void>,
    pub init_conf: ::std::option::Option<extern "C" fn
                                             (cycle: *mut ngx_cycle_t,
                                              conf: *mut ::libc::c_void)
                                             -> *mut ::libc::c_char>,
    pub actions: ngx_event_actions_t,
}
impl ::std::default::Default for Struct_Unnamed197 {
    fn default() -> Struct_Unnamed197 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_event_module_t = Struct_Unnamed197;
pub type ngx_event_busy_lock_ctx_t = Struct_ngx_event_busy_lock_ctx_s;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_event_busy_lock_ctx_s {
    pub event: *mut ngx_event_t,
    pub handler: ngx_event_handler_pt,
    pub data: *mut ::libc::c_void,
    pub timer: ngx_msec_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub md5: *mut ::libc::c_char,
    pub slot: ngx_int_t,
    pub next: *mut ngx_event_busy_lock_ctx_t,
}
impl ::std::default::Default for Struct_ngx_event_busy_lock_ctx_s {
    fn default() -> Struct_ngx_event_busy_lock_ctx_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed198 {
    pub md5_mask: *mut u_char,
    pub md5: *mut ::libc::c_char,
    pub cacheable: ngx_uint_t,
    pub busy: ngx_uint_t,
    pub max_busy: ngx_uint_t,
    pub waiting: ngx_uint_t,
    pub max_waiting: ngx_uint_t,
    pub events: *mut ngx_event_busy_lock_ctx_t,
    pub last: *mut ngx_event_busy_lock_ctx_t,
}
impl ::std::default::Default for Struct_Unnamed198 {
    fn default() -> Struct_Unnamed198 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_event_busy_lock_t = Struct_Unnamed198;
pub type ngx_peer_connection_t = Struct_ngx_peer_connection_s;
pub type ngx_event_get_peer_pt =
    ::std::option::Option<extern "C" fn
                              (pc: *mut ngx_peer_connection_t,
                               data: *mut ::libc::c_void) -> ngx_int_t>;
pub type ngx_event_free_peer_pt =
    ::std::option::Option<extern "C" fn
                              (pc: *mut ngx_peer_connection_t,
                               data: *mut ::libc::c_void, state: ngx_uint_t)
                              -> ()>;
pub type ngx_event_set_peer_session_pt =
    ::std::option::Option<extern "C" fn
                              (pc: *mut ngx_peer_connection_t,
                               data: *mut ::libc::c_void) -> ngx_int_t>;
pub type ngx_event_save_peer_session_pt =
    ::std::option::Option<extern "C" fn
                              (pc: *mut ngx_peer_connection_t,
                               data: *mut ::libc::c_void) -> ()>;
#[repr(C)]
pub struct Struct_ngx_peer_connection_s {
    pub connection: *mut ngx_connection_t,
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub name: *mut ngx_str_t,
    pub tries: ngx_uint_t,
    pub start_time: ngx_msec_t,
    pub get: ngx_event_get_peer_pt,
    pub free: ngx_event_free_peer_pt,
    pub data: *mut ::libc::c_void,
    pub set_session: ngx_event_set_peer_session_pt,
    pub save_session: ngx_event_save_peer_session_pt,
    pub local: *mut ngx_addr_t,
    pub rcvbuf: ::libc::c_int,
    pub log: *mut ngx_log_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_peer_connection_s {
    fn default() -> Struct_ngx_peer_connection_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_event_pipe_t = Struct_ngx_event_pipe_s;
pub type ngx_event_pipe_input_filter_pt =
    ::std::option::Option<extern "C" fn
                              (p: *mut ngx_event_pipe_t, buf: *mut ngx_buf_t)
                              -> ngx_int_t>;
pub type ngx_event_pipe_output_filter_pt =
    ::std::option::Option<extern "C" fn
                              (data: *mut ::libc::c_void,
                               chain: *mut ngx_chain_t) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_event_pipe_s {
    pub upstream: *mut ngx_connection_t,
    pub downstream: *mut ngx_connection_t,
    pub free_raw_bufs: *mut ngx_chain_t,
    pub _in: *mut ngx_chain_t,
    pub last_in: *mut *mut ngx_chain_t,
    pub out: *mut ngx_chain_t,
    pub free: *mut ngx_chain_t,
    pub busy: *mut ngx_chain_t,
    pub input_filter: ngx_event_pipe_input_filter_pt,
    pub input_ctx: *mut ::libc::c_void,
    pub output_filter: ngx_event_pipe_output_filter_pt,
    pub output_ctx: *mut ::libc::c_void,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub allocated: ngx_int_t,
    pub bufs: ngx_bufs_t,
    pub tag: ngx_buf_tag_t,
    pub busy_size: ssize_t,
    pub read_length: off_t,
    pub length: off_t,
    pub max_temp_file_size: off_t,
    pub temp_file_write_size: ssize_t,
    pub read_timeout: ngx_msec_t,
    pub send_timeout: ngx_msec_t,
    pub send_lowat: ssize_t,
    pub pool: *mut ngx_pool_t,
    pub log: *mut ngx_log_t,
    pub preread_bufs: *mut ngx_chain_t,
    pub preread_size: size_t,
    pub buf_to_file: *mut ngx_buf_t,
    pub limit_rate: size_t,
    pub start_sec: time_t,
    pub temp_file: *mut ngx_temp_file_t,
    pub num: ::libc::c_int,
}
impl ::std::default::Default for Struct_ngx_event_pipe_s {
    fn default() -> Struct_ngx_event_pipe_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed199 {
    pub bl_time: ngx_msec_t,
    pub bl_state: ngx_uint_t,
    pub status: ngx_uint_t,
    pub response_sec: time_t,
    pub response_msec: ngx_uint_t,
    pub header_sec: time_t,
    pub header_msec: ngx_uint_t,
    pub response_length: off_t,
    pub peer: *mut ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed199 {
    fn default() -> Struct_Unnamed199 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_state_t = Struct_Unnamed199;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed200 {
    pub headers_in_hash: ngx_hash_t,
    pub upstreams: ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed200 {
    fn default() -> Struct_Unnamed200 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_main_conf_t = Struct_Unnamed200;
pub type ngx_http_upstream_srv_conf_t = Struct_ngx_http_upstream_srv_conf_s;
pub type ngx_http_upstream_init_pt =
    ::std::option::Option<extern "C" fn
                              (cf: *mut ngx_conf_t,
                               us: *mut ngx_http_upstream_srv_conf_t)
                              -> ngx_int_t>;
pub type ngx_http_upstream_init_peer_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               us: *mut ngx_http_upstream_srv_conf_t)
                              -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed201 {
    pub init_upstream: ngx_http_upstream_init_pt,
    pub init: ngx_http_upstream_init_peer_pt,
    pub data: *mut ::libc::c_void,
}
impl ::std::default::Default for Struct_Unnamed201 {
    fn default() -> Struct_Unnamed201 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_peer_t = Struct_Unnamed201;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed202 {
    pub name: ngx_str_t,
    pub addrs: *mut ngx_addr_t,
    pub naddrs: ngx_uint_t,
    pub weight: ngx_uint_t,
    pub max_fails: ngx_uint_t,
    pub fail_timeout: time_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed202 {
    fn default() -> Struct_Unnamed202 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_server_t = Struct_Unnamed202;
#[repr(C)]
pub struct Struct_ngx_http_upstream_srv_conf_s {
    pub peer: ngx_http_upstream_peer_t,
    pub srv_conf: *mut *mut ::libc::c_void,
    pub servers: *mut ngx_array_t,
    pub flags: ngx_uint_t,
    pub host: ngx_str_t,
    pub file_name: *mut u_char,
    pub line: ngx_uint_t,
    pub port: in_port_t,
    pub default_port: in_port_t,
    pub no_port: ngx_uint_t,
}
impl ::std::default::Default for Struct_ngx_http_upstream_srv_conf_s {
    fn default() -> Struct_ngx_http_upstream_srv_conf_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed203 {
    pub addr: *mut ngx_addr_t,
    pub value: *mut ngx_http_complex_value_t,
}
impl ::std::default::Default for Struct_Unnamed203 {
    fn default() -> Struct_Unnamed203 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_local_t = Struct_Unnamed203;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed204 {
    pub upstream: *mut ngx_http_upstream_srv_conf_t,
    pub connect_timeout: ngx_msec_t,
    pub send_timeout: ngx_msec_t,
    pub read_timeout: ngx_msec_t,
    pub timeout: ngx_msec_t,
    pub next_upstream_timeout: ngx_msec_t,
    pub send_lowat: size_t,
    pub buffer_size: size_t,
    pub limit_rate: size_t,
    pub busy_buffers_size: size_t,
    pub max_temp_file_size: size_t,
    pub temp_file_write_size: size_t,
    pub busy_buffers_size_conf: size_t,
    pub max_temp_file_size_conf: size_t,
    pub temp_file_write_size_conf: size_t,
    pub bufs: ngx_bufs_t,
    pub ignore_headers: ngx_uint_t,
    pub next_upstream: ngx_uint_t,
    pub store_access: ngx_uint_t,
    pub next_upstream_tries: ngx_uint_t,
    pub buffering: ngx_flag_t,
    pub pass_request_headers: ngx_flag_t,
    pub pass_request_body: ngx_flag_t,
    pub ignore_client_abort: ngx_flag_t,
    pub intercept_errors: ngx_flag_t,
    pub cyclic_temp_file: ngx_flag_t,
    pub force_ranges: ngx_flag_t,
    pub temp_path: *mut ngx_path_t,
    pub hide_headers_hash: ngx_hash_t,
    pub hide_headers: *mut ngx_array_t,
    pub pass_headers: *mut ngx_array_t,
    pub local: *mut ngx_http_upstream_local_t,
    pub cache_zone: *mut ngx_shm_zone_t,
    pub cache_value: *mut ngx_http_complex_value_t,
    pub cache_min_uses: ngx_uint_t,
    pub cache_use_stale: ngx_uint_t,
    pub cache_methods: ngx_uint_t,
    pub cache_lock: ngx_flag_t,
    pub cache_lock_timeout: ngx_msec_t,
    pub cache_lock_age: ngx_msec_t,
    pub cache_revalidate: ngx_flag_t,
    pub cache_valid: *mut ngx_array_t,
    pub cache_bypass: *mut ngx_array_t,
    pub no_cache: *mut ngx_array_t,
    pub store_lengths: *mut ngx_array_t,
    pub store_values: *mut ngx_array_t,
    pub _bindgen_bitfield_1_: ::libc::c_int,
    pub _bindgen_bitfield_2_: ::libc::c_uint,
    pub ssl: *mut ngx_ssl_t,
    pub ssl_session_reuse: ngx_flag_t,
    pub ssl_name: *mut ngx_http_complex_value_t,
    pub ssl_server_name: ngx_flag_t,
    pub ssl_verify: ngx_flag_t,
    pub module: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed204 {
    fn default() -> Struct_Unnamed204 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_conf_t = Struct_Unnamed204;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed205 {
    pub name: ngx_str_t,
    pub handler: ngx_http_header_handler_pt,
    pub offset: ngx_uint_t,
    pub copy_handler: ngx_http_header_handler_pt,
    pub conf: ngx_uint_t,
    pub redirect: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed205 {
    fn default() -> Struct_Unnamed205 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_header_t = Struct_Unnamed205;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed206 {
    pub headers: ngx_list_t,
    pub status_n: ngx_uint_t,
    pub status_line: ngx_str_t,
    pub status: *mut ngx_table_elt_t,
    pub date: *mut ngx_table_elt_t,
    pub server: *mut ngx_table_elt_t,
    pub connection: *mut ngx_table_elt_t,
    pub expires: *mut ngx_table_elt_t,
    pub etag: *mut ngx_table_elt_t,
    pub x_accel_expires: *mut ngx_table_elt_t,
    pub x_accel_redirect: *mut ngx_table_elt_t,
    pub x_accel_limit_rate: *mut ngx_table_elt_t,
    pub content_type: *mut ngx_table_elt_t,
    pub content_length: *mut ngx_table_elt_t,
    pub last_modified: *mut ngx_table_elt_t,
    pub location: *mut ngx_table_elt_t,
    pub accept_ranges: *mut ngx_table_elt_t,
    pub www_authenticate: *mut ngx_table_elt_t,
    pub transfer_encoding: *mut ngx_table_elt_t,
    pub vary: *mut ngx_table_elt_t,
    pub content_encoding: *mut ngx_table_elt_t,
    pub cache_control: ngx_array_t,
    pub cookies: ngx_array_t,
    pub content_length_n: off_t,
    pub last_modified_time: time_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed206 {
    fn default() -> Struct_Unnamed206 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_headers_in_t = Struct_Unnamed206;
#[repr(C)]
pub struct Struct_Unnamed207 {
    pub host: ngx_str_t,
    pub port: in_port_t,
    pub no_port: ngx_uint_t,
    pub naddrs: ngx_uint_t,
    pub addrs: *mut ngx_addr_t,
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub ctx: *mut ngx_resolver_ctx_t,
}
impl ::std::default::Default for Struct_Unnamed207 {
    fn default() -> Struct_Unnamed207 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_resolved_t = Struct_Unnamed207;
pub type ngx_http_upstream_handler_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               u: *mut ngx_http_upstream_t) -> ()>;
#[repr(C)]
pub struct Struct_ngx_http_upstream_s {
    pub read_event_handler: ngx_http_upstream_handler_pt,
    pub write_event_handler: ngx_http_upstream_handler_pt,
    pub peer: ngx_peer_connection_t,
    pub pipe: *mut ngx_event_pipe_t,
    pub request_bufs: *mut ngx_chain_t,
    pub output: ngx_output_chain_ctx_t,
    pub writer: ngx_chain_writer_ctx_t,
    pub conf: *mut ngx_http_upstream_conf_t,
    pub caches: *mut ngx_array_t,
    pub headers_in: ngx_http_upstream_headers_in_t,
    pub resolved: *mut ngx_http_upstream_resolved_t,
    pub from_client: ngx_buf_t,
    pub buffer: ngx_buf_t,
    pub length: off_t,
    pub out_bufs: *mut ngx_chain_t,
    pub busy_bufs: *mut ngx_chain_t,
    pub free_bufs: *mut ngx_chain_t,
    pub input_filter_init: ::std::option::Option<extern "C" fn
                                                     (data:
                                                          *mut ::libc::c_void)
                                                     -> ngx_int_t>,
    pub input_filter: ::std::option::Option<extern "C" fn
                                                (data: *mut ::libc::c_void,
                                                 bytes: ssize_t)
                                                -> ngx_int_t>,
    pub input_filter_ctx: *mut ::libc::c_void,
    pub create_key: ::std::option::Option<extern "C" fn
                                              (r: *mut ngx_http_request_t)
                                              -> ngx_int_t>,
    pub create_request: ::std::option::Option<extern "C" fn
                                                  (r: *mut ngx_http_request_t)
                                                  -> ngx_int_t>,
    pub reinit_request: ::std::option::Option<extern "C" fn
                                                  (r: *mut ngx_http_request_t)
                                                  -> ngx_int_t>,
    pub process_header: ::std::option::Option<extern "C" fn
                                                  (r: *mut ngx_http_request_t)
                                                  -> ngx_int_t>,
    pub abort_request: ::std::option::Option<extern "C" fn
                                                 (r: *mut ngx_http_request_t)
                                                 -> ()>,
    pub finalize_request: ::std::option::Option<extern "C" fn
                                                    (r:
                                                         *mut ngx_http_request_t,
                                                     rc: ngx_int_t) -> ()>,
    pub rewrite_redirect: ::std::option::Option<extern "C" fn
                                                    (r:
                                                         *mut ngx_http_request_t,
                                                     h: *mut ngx_table_elt_t,
                                                     prefix: size_t)
                                                    -> ngx_int_t>,
    pub rewrite_cookie: ::std::option::Option<extern "C" fn
                                                  (r: *mut ngx_http_request_t,
                                                   h: *mut ngx_table_elt_t)
                                                  -> ngx_int_t>,
    pub timeout: ngx_msec_t,
    pub state: *mut ngx_http_upstream_state_t,
    pub method: ngx_str_t,
    pub schema: ngx_str_t,
    pub uri: ngx_str_t,
    pub ssl_name: ngx_str_t,
    pub cleanup: *mut ngx_http_cleanup_pt,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_http_upstream_s {
    fn default() -> Struct_ngx_http_upstream_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed208 {
    pub status: ngx_uint_t,
    pub mask: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed208 {
    fn default() -> Struct_Unnamed208 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_next_t = Struct_Unnamed208;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed209 {
    pub key: ngx_str_t,
    pub value: ngx_str_t,
    pub skip_empty: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed209 {
    fn default() -> Struct_Unnamed209 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_param_t = Struct_Unnamed209;
#[repr(C)]
pub struct Struct_Unnamed210 {
    pub sockaddr: *mut Struct_sockaddr,
    pub socklen: socklen_t,
    pub name: ngx_str_t,
    pub server: ngx_str_t,
    pub current_weight: ngx_int_t,
    pub effective_weight: ngx_int_t,
    pub weight: ngx_int_t,
    pub fails: ngx_uint_t,
    pub accessed: time_t,
    pub checked: time_t,
    pub max_fails: ngx_uint_t,
    pub fail_timeout: time_t,
    pub down: ngx_uint_t,
    pub ssl_session: *mut SSL_SESSION,
}
impl ::std::default::Default for Struct_Unnamed210 {
    fn default() -> Struct_Unnamed210 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_rr_peer_t = Struct_Unnamed210;
pub type ngx_http_upstream_rr_peers_t = Struct_ngx_http_upstream_rr_peers_s;
#[repr(C)]
pub struct Struct_ngx_http_upstream_rr_peers_s {
    pub number: ngx_uint_t,
    pub total_weight: ngx_uint_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub name: *mut ngx_str_t,
    pub next: *mut ngx_http_upstream_rr_peers_t,
    pub peer: [ngx_http_upstream_rr_peer_t; 1usize],
}
impl ::std::default::Default for Struct_ngx_http_upstream_rr_peers_s {
    fn default() -> Struct_ngx_http_upstream_rr_peers_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed211 {
    pub peers: *mut ngx_http_upstream_rr_peers_t,
    pub current: ngx_uint_t,
    pub tried: *mut uintptr_t,
    pub data: uintptr_t,
}
impl ::std::default::Default for Struct_Unnamed211 {
    fn default() -> Struct_Unnamed211 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_upstream_rr_peer_data_t = Struct_Unnamed211;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed212 {
    pub md5_mask: *mut u_char,
    pub md5: *mut ::libc::c_char,
    pub cacheable: ::libc::c_int,
    pub busy: ::libc::c_int,
    pub max_busy: ::libc::c_int,
    pub waiting: ::libc::c_int,
    pub max_waiting: ::libc::c_int,
    pub timeout: time_t,
    pub mutex: *mut ngx_event_mutex_t,
}
impl ::std::default::Default for Struct_Unnamed212 {
    fn default() -> Struct_Unnamed212 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_busy_lock_t = Struct_Unnamed212;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed213 {
    pub time: time_t,
    pub event: *mut ngx_event_t,
    pub event_handler: ::std::option::Option<extern "C" fn
                                                 (ev: *mut ngx_event_t)
                                                 -> ()>,
    pub md5: *mut u_char,
    pub slot: ::libc::c_int,
}
impl ::std::default::Default for Struct_Unnamed213 {
    fn default() -> Struct_Unnamed213 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_busy_lock_ctx_t = Struct_Unnamed213;
pub type ngx_http_location_tree_node_t = Struct_ngx_http_location_tree_node_s;
pub type ngx_http_core_loc_conf_t = Struct_ngx_http_core_loc_conf_s;
#[repr(C)]
pub struct Struct_Unnamed214 {
    pub u: Union_Unnamed215,
    pub socklen: socklen_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub backlog: ::libc::c_int,
    pub rcvbuf: ::libc::c_int,
    pub sndbuf: ::libc::c_int,
    pub fastopen: ::libc::c_int,
    pub tcp_keepidle: ::libc::c_int,
    pub tcp_keepintvl: ::libc::c_int,
    pub tcp_keepcnt: ::libc::c_int,
    pub deferred_accept: ngx_uint_t,
    pub addr: [u_char; 114usize],
}
impl ::std::default::Default for Struct_Unnamed214 {
    fn default() -> Struct_Unnamed214 { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed215 {
    pub _bindgen_data_: [u32; 28usize],
}
impl Union_Unnamed215 {
    pub unsafe fn sockaddr(&mut self) -> *mut Struct_sockaddr {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn sockaddr_in(&mut self) -> *mut Struct_sockaddr_in {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn sockaddr_un(&mut self) -> *mut Struct_sockaddr_un {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn sockaddr_data(&mut self) -> *mut [u_char; 110usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl ::std::default::Default for Union_Unnamed215 {
    fn default() -> Union_Unnamed215 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_listen_opt_t = Struct_Unnamed214;
pub type Enum_Unnamed216 = ::libc::c_uint;
pub const NGX_HTTP_POST_READ_PHASE: ::libc::c_uint = 0;
pub const NGX_HTTP_SERVER_REWRITE_PHASE: ::libc::c_uint = 1;
pub const NGX_HTTP_FIND_CONFIG_PHASE: ::libc::c_uint = 2;
pub const NGX_HTTP_REWRITE_PHASE: ::libc::c_uint = 3;
pub const NGX_HTTP_POST_REWRITE_PHASE: ::libc::c_uint = 4;
pub const NGX_HTTP_PREACCESS_PHASE: ::libc::c_uint = 5;
pub const NGX_HTTP_ACCESS_PHASE: ::libc::c_uint = 6;
pub const NGX_HTTP_POST_ACCESS_PHASE: ::libc::c_uint = 7;
pub const NGX_HTTP_TRY_FILES_PHASE: ::libc::c_uint = 8;
pub const NGX_HTTP_CONTENT_PHASE: ::libc::c_uint = 9;
pub const NGX_HTTP_LOG_PHASE: ::libc::c_uint = 10;
pub type ngx_http_phases = Enum_Unnamed216;
pub type ngx_http_phase_handler_t = Struct_ngx_http_phase_handler_s;
pub type ngx_http_phase_handler_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               ph: *mut ngx_http_phase_handler_t)
                              -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_phase_handler_s {
    pub checker: ngx_http_phase_handler_pt,
    pub handler: ngx_http_handler_pt,
    pub next: ngx_uint_t,
}
impl ::std::default::Default for Struct_ngx_http_phase_handler_s {
    fn default() -> Struct_ngx_http_phase_handler_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed217 {
    pub handlers: *mut ngx_http_phase_handler_t,
    pub server_rewrite_index: ngx_uint_t,
    pub location_rewrite_index: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed217 {
    fn default() -> Struct_Unnamed217 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_phase_engine_t = Struct_Unnamed217;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed218 {
    pub handlers: ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed218 {
    fn default() -> Struct_Unnamed218 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_phase_t = Struct_Unnamed218;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed219 {
    pub servers: ngx_array_t,
    pub phase_engine: ngx_http_phase_engine_t,
    pub headers_in_hash: ngx_hash_t,
    pub variables_hash: ngx_hash_t,
    pub variables: ngx_array_t,
    pub ncaptures: ngx_uint_t,
    pub server_names_hash_max_size: ngx_uint_t,
    pub server_names_hash_bucket_size: ngx_uint_t,
    pub variables_hash_max_size: ngx_uint_t,
    pub variables_hash_bucket_size: ngx_uint_t,
    pub variables_keys: *mut ngx_hash_keys_arrays_t,
    pub ports: *mut ngx_array_t,
    pub try_files: ngx_uint_t,
    pub phases: [ngx_http_phase_t; 11usize],
}
impl ::std::default::Default for Struct_Unnamed219 {
    fn default() -> Struct_Unnamed219 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_core_main_conf_t = Struct_Unnamed219;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed220 {
    pub server_names: ngx_array_t,
    pub ctx: *mut ngx_http_conf_ctx_t,
    pub server_name: ngx_str_t,
    pub connection_pool_size: size_t,
    pub request_pool_size: size_t,
    pub client_header_buffer_size: size_t,
    pub large_client_header_buffers: ngx_bufs_t,
    pub client_header_timeout: ngx_msec_t,
    pub ignore_invalid_headers: ngx_flag_t,
    pub merge_slashes: ngx_flag_t,
    pub underscores_in_headers: ngx_flag_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub named_locations: *mut *mut ngx_http_core_loc_conf_t,
}
impl ::std::default::Default for Struct_Unnamed220 {
    fn default() -> Struct_Unnamed220 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_core_srv_conf_t = Struct_Unnamed220;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed221 {
    pub regex: *mut ngx_http_regex_t,
    pub server: *mut ngx_http_core_srv_conf_t,
    pub name: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed221 {
    fn default() -> Struct_Unnamed221 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_server_name_t = Struct_Unnamed221;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed222 {
    pub names: ngx_hash_combined_t,
    pub nregex: ngx_uint_t,
    pub regex: *mut ngx_http_server_name_t,
}
impl ::std::default::Default for Struct_Unnamed222 {
    fn default() -> Struct_Unnamed222 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_virtual_names_t = Struct_Unnamed222;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_addr_conf_s {
    pub default_server: *mut ngx_http_core_srv_conf_t,
    pub virtual_names: *mut ngx_http_virtual_names_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_http_addr_conf_s {
    fn default() -> Struct_ngx_http_addr_conf_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct Struct_Unnamed223 {
    pub addr: in_addr_t,
    pub conf: ngx_http_addr_conf_t,
}
impl ::std::default::Default for Struct_Unnamed223 {
    fn default() -> Struct_Unnamed223 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_in_addr_t = Struct_Unnamed223;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed224 {
    pub addrs: *mut ::libc::c_void,
    pub naddrs: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed224 {
    fn default() -> Struct_Unnamed224 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_port_t = Struct_Unnamed224;
#[repr(C)]
pub struct Struct_Unnamed225 {
    pub family: ngx_int_t,
    pub port: in_port_t,
    pub addrs: ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed225 {
    fn default() -> Struct_Unnamed225 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_conf_port_t = Struct_Unnamed225;
#[repr(C)]
pub struct Struct_Unnamed226 {
    pub opt: ngx_http_listen_opt_t,
    pub hash: ngx_hash_t,
    pub wc_head: *mut ngx_hash_wildcard_t,
    pub wc_tail: *mut ngx_hash_wildcard_t,
    pub nregex: ngx_uint_t,
    pub regex: *mut ngx_http_server_name_t,
    pub default_server: *mut ngx_http_core_srv_conf_t,
    pub servers: ngx_array_t,
}
impl ::std::default::Default for Struct_Unnamed226 {
    fn default() -> Struct_Unnamed226 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_conf_addr_t = Struct_Unnamed226;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed227 {
    pub status: ngx_int_t,
    pub overwrite: ngx_int_t,
    pub value: ngx_http_complex_value_t,
    pub args: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed227 {
    fn default() -> Struct_Unnamed227 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_err_page_t = Struct_Unnamed227;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed228 {
    pub lengths: *mut ngx_array_t,
    pub values: *mut ngx_array_t,
    pub name: ngx_str_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed228 {
    fn default() -> Struct_Unnamed228 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_try_file_t = Struct_Unnamed228;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_core_loc_conf_s {
    pub name: ngx_str_t,
    pub regex: *mut ngx_http_regex_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub static_locations: *mut ngx_http_location_tree_node_t,
    pub regex_locations: *mut *mut ngx_http_core_loc_conf_t,
    pub loc_conf: *mut *mut ::libc::c_void,
    pub limit_except: uint32_t,
    pub limit_except_loc_conf: *mut *mut ::libc::c_void,
    pub handler: ngx_http_handler_pt,
    pub alias: size_t,
    pub root: ngx_str_t,
    pub post_action: ngx_str_t,
    pub root_lengths: *mut ngx_array_t,
    pub root_values: *mut ngx_array_t,
    pub types: *mut ngx_array_t,
    pub types_hash: ngx_hash_t,
    pub default_type: ngx_str_t,
    pub client_max_body_size: off_t,
    pub directio: off_t,
    pub directio_alignment: off_t,
    pub client_body_buffer_size: size_t,
    pub send_lowat: size_t,
    pub postpone_output: size_t,
    pub limit_rate: size_t,
    pub limit_rate_after: size_t,
    pub sendfile_max_chunk: size_t,
    pub read_ahead: size_t,
    pub client_body_timeout: ngx_msec_t,
    pub send_timeout: ngx_msec_t,
    pub keepalive_timeout: ngx_msec_t,
    pub lingering_time: ngx_msec_t,
    pub lingering_timeout: ngx_msec_t,
    pub resolver_timeout: ngx_msec_t,
    pub resolver: *mut ngx_resolver_t,
    pub keepalive_header: time_t,
    pub keepalive_requests: ngx_uint_t,
    pub keepalive_disable: ngx_uint_t,
    pub satisfy: ngx_uint_t,
    pub lingering_close: ngx_uint_t,
    pub if_modified_since: ngx_uint_t,
    pub max_ranges: ngx_uint_t,
    pub client_body_in_file_only: ngx_uint_t,
    pub client_body_in_single_buffer: ngx_flag_t,
    pub internal: ngx_flag_t,
    pub sendfile: ngx_flag_t,
    pub tcp_nopush: ngx_flag_t,
    pub tcp_nodelay: ngx_flag_t,
    pub reset_timedout_connection: ngx_flag_t,
    pub server_name_in_redirect: ngx_flag_t,
    pub port_in_redirect: ngx_flag_t,
    pub msie_padding: ngx_flag_t,
    pub msie_refresh: ngx_flag_t,
    pub log_not_found: ngx_flag_t,
    pub log_subrequest: ngx_flag_t,
    pub recursive_error_pages: ngx_flag_t,
    pub server_tokens: ngx_flag_t,
    pub chunked_transfer_encoding: ngx_flag_t,
    pub etag: ngx_flag_t,
    pub gzip_vary: ngx_flag_t,
    pub gzip_http_version: ngx_uint_t,
    pub gzip_proxied: ngx_uint_t,
    pub gzip_disable: *mut ngx_array_t,
    pub disable_symlinks: ngx_uint_t,
    pub disable_symlinks_from: *mut ngx_http_complex_value_t,
    pub error_pages: *mut ngx_array_t,
    pub try_files: *mut ngx_http_try_file_t,
    pub client_body_temp_path: *mut ngx_path_t,
    pub open_file_cache: *mut ngx_open_file_cache_t,
    pub open_file_cache_valid: time_t,
    pub open_file_cache_min_uses: ngx_uint_t,
    pub open_file_cache_errors: ngx_flag_t,
    pub open_file_cache_events: ngx_flag_t,
    pub error_log: *mut ngx_log_t,
    pub types_hash_max_size: ngx_uint_t,
    pub types_hash_bucket_size: ngx_uint_t,
    pub locations: *mut ngx_queue_t,
}
impl ::std::default::Default for Struct_ngx_http_core_loc_conf_s {
    fn default() -> Struct_ngx_http_core_loc_conf_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed229 {
    pub queue: ngx_queue_t,
    pub exact: *mut ngx_http_core_loc_conf_t,
    pub inclusive: *mut ngx_http_core_loc_conf_t,
    pub name: *mut ngx_str_t,
    pub file_name: *mut u_char,
    pub line: ngx_uint_t,
    pub list: ngx_queue_t,
}
impl ::std::default::Default for Struct_Unnamed229 {
    fn default() -> Struct_Unnamed229 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_location_queue_t = Struct_Unnamed229;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_location_tree_node_s {
    pub left: *mut ngx_http_location_tree_node_t,
    pub right: *mut ngx_http_location_tree_node_t,
    pub tree: *mut ngx_http_location_tree_node_t,
    pub exact: *mut ngx_http_core_loc_conf_t,
    pub inclusive: *mut ngx_http_core_loc_conf_t,
    pub auto_redirect: u_char,
    pub len: u_char,
    pub name: [u_char; 1usize],
}
impl ::std::default::Default for Struct_ngx_http_location_tree_node_s {
    fn default() -> Struct_ngx_http_location_tree_node_s {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type ngx_http_output_header_filter_pt =
    ::std::option::Option<extern "C" fn(r: *mut ngx_http_request_t)
                              -> ngx_int_t>;
pub type ngx_http_output_body_filter_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               chain: *mut ngx_chain_t) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed230 {
    pub status: ngx_uint_t,
    pub valid: time_t,
}
impl ::std::default::Default for Struct_Unnamed230 {
    fn default() -> Struct_Unnamed230 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_cache_valid_t = Struct_Unnamed230;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed231 {
    pub node: ngx_rbtree_node_t,
    pub queue: ngx_queue_t,
    pub key: [u_char; 8usize],
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub _bindgen_bitfield_2_: ::libc::c_uint,
    pub uniq: ngx_file_uniq_t,
    pub expire: time_t,
    pub valid_sec: time_t,
    pub body_start: size_t,
    pub fs_size: off_t,
    pub lock_time: ngx_msec_t,
}
impl ::std::default::Default for Struct_Unnamed231 {
    fn default() -> Struct_Unnamed231 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_file_cache_node_t = Struct_Unnamed231;
#[repr(C)]
pub struct Struct_ngx_http_cache_s {
    pub file: ngx_file_t,
    pub keys: ngx_array_t,
    pub crc32: uint32_t,
    pub key: [u_char; 16usize],
    pub main: [u_char; 16usize],
    pub uniq: ngx_file_uniq_t,
    pub valid_sec: time_t,
    pub last_modified: time_t,
    pub date: time_t,
    pub etag: ngx_str_t,
    pub vary: ngx_str_t,
    pub variant: [u_char; 16usize],
    pub header_start: size_t,
    pub body_start: size_t,
    pub length: off_t,
    pub fs_size: off_t,
    pub min_uses: ngx_uint_t,
    pub error: ngx_uint_t,
    pub valid_msec: ngx_uint_t,
    pub buf: *mut ngx_buf_t,
    pub file_cache: *mut ngx_http_file_cache_t,
    pub node: *mut ngx_http_file_cache_node_t,
    pub lock_timeout: ngx_msec_t,
    pub lock_age: ngx_msec_t,
    pub lock_time: ngx_msec_t,
    pub wait_time: ngx_msec_t,
    pub wait_event: ngx_event_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_ngx_http_cache_s {
    fn default() -> Struct_ngx_http_cache_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed232 {
    pub version: ngx_uint_t,
    pub valid_sec: time_t,
    pub last_modified: time_t,
    pub date: time_t,
    pub crc32: uint32_t,
    pub valid_msec: u_short,
    pub header_start: u_short,
    pub body_start: u_short,
    pub etag_len: u_char,
    pub etag: [u_char; 42usize],
    pub vary_len: u_char,
    pub vary: [u_char; 42usize],
    pub variant: [u_char; 16usize],
}
impl ::std::default::Default for Struct_Unnamed232 {
    fn default() -> Struct_Unnamed232 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_file_cache_header_t = Struct_Unnamed232;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed233 {
    pub rbtree: ngx_rbtree_t,
    pub sentinel: ngx_rbtree_node_t,
    pub queue: ngx_queue_t,
    pub cold: ngx_atomic_t,
    pub loading: ngx_atomic_t,
    pub size: off_t,
}
impl ::std::default::Default for Struct_Unnamed233 {
    fn default() -> Struct_Unnamed233 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_file_cache_sh_t = Struct_Unnamed233;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_file_cache_s {
    pub sh: *mut ngx_http_file_cache_sh_t,
    pub shpool: *mut ngx_slab_pool_t,
    pub path: *mut ngx_path_t,
    pub temp_path: *mut ngx_path_t,
    pub max_size: off_t,
    pub bsize: size_t,
    pub inactive: time_t,
    pub files: ngx_uint_t,
    pub loader_files: ngx_uint_t,
    pub last: ngx_msec_t,
    pub loader_sleep: ngx_msec_t,
    pub loader_threshold: ngx_msec_t,
    pub shm_zone: *mut ngx_shm_zone_t,
}
impl ::std::default::Default for Struct_ngx_http_file_cache_s {
    fn default() -> Struct_ngx_http_file_cache_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed234 {
    pub hash: ngx_hash_t,
    pub commands: ngx_hash_keys_arrays_t,
}
impl ::std::default::Default for Struct_Unnamed234 {
    fn default() -> Struct_Unnamed234 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ssi_main_conf_t = Struct_Unnamed234;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed235 {
    pub buf: *mut ngx_buf_t,
    pub pos: *mut u_char,
    pub copy_start: *mut u_char,
    pub copy_end: *mut u_char,
    pub key: ngx_uint_t,
    pub command: ngx_str_t,
    pub params: ngx_array_t,
    pub param: *mut ngx_table_elt_t,
    pub params_array: [ngx_table_elt_t; 4usize],
    pub _in: *mut ngx_chain_t,
    pub out: *mut ngx_chain_t,
    pub last_out: *mut *mut ngx_chain_t,
    pub busy: *mut ngx_chain_t,
    pub free: *mut ngx_chain_t,
    pub state: ngx_uint_t,
    pub saved_state: ngx_uint_t,
    pub saved: size_t,
    pub looked: size_t,
    pub value_len: size_t,
    pub variables: *mut ngx_list_t,
    pub blocks: *mut ngx_array_t,
    pub ncaptures: ngx_uint_t,
    pub captures: *mut ::libc::c_int,
    pub captures_data: *mut u_char,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
    pub wait: *mut ngx_http_request_t,
    pub value_buf: *mut ::libc::c_void,
    pub timefmt: ngx_str_t,
    pub errmsg: ngx_str_t,
}
impl ::std::default::Default for Struct_Unnamed235 {
    fn default() -> Struct_Unnamed235 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ssi_ctx_t = Struct_Unnamed235;
pub type ngx_http_ssi_command_pt =
    ::std::option::Option<extern "C" fn
                              (r: *mut ngx_http_request_t,
                               ctx: *mut ngx_http_ssi_ctx_t,
                               arg1: *mut *mut ngx_str_t) -> ngx_int_t>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed236 {
    pub name: ngx_str_t,
    pub index: ngx_uint_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed236 {
    fn default() -> Struct_Unnamed236 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ssi_param_t = Struct_Unnamed236;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed237 {
    pub name: ngx_str_t,
    pub handler: ngx_http_ssi_command_pt,
    pub params: *mut ngx_http_ssi_param_t,
    pub _bindgen_bitfield_1_: ::libc::c_uint,
}
impl ::std::default::Default for Struct_Unnamed237 {
    fn default() -> Struct_Unnamed237 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ssi_command_t = Struct_Unnamed237;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed238 {
    pub enable: ngx_flag_t,
    pub ssl: ngx_ssl_t,
    pub prefer_server_ciphers: ngx_flag_t,
    pub protocols: ngx_uint_t,
    pub verify: ngx_uint_t,
    pub verify_depth: ngx_uint_t,
    pub buffer_size: size_t,
    pub builtin_session_cache: ssize_t,
    pub session_timeout: time_t,
    pub certificate: ngx_str_t,
    pub certificate_key: ngx_str_t,
    pub dhparam: ngx_str_t,
    pub ecdh_curve: ngx_str_t,
    pub client_certificate: ngx_str_t,
    pub trusted_certificate: ngx_str_t,
    pub crl: ngx_str_t,
    pub ciphers: ngx_str_t,
    pub passwords: *mut ngx_array_t,
    pub shm_zone: *mut ngx_shm_zone_t,
    pub session_tickets: ngx_flag_t,
    pub session_ticket_keys: *mut ngx_array_t,
    pub stapling: ngx_flag_t,
    pub stapling_verify: ngx_flag_t,
    pub stapling_file: ngx_str_t,
    pub stapling_responder: ngx_str_t,
    pub file: *mut u_char,
    pub line: ngx_uint_t,
}
impl ::std::default::Default for Struct_Unnamed238 {
    fn default() -> Struct_Unnamed238 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_ssl_srv_conf_t = Struct_Unnamed238;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_log_ctx_s {
    pub connection: *mut ngx_connection_t,
    pub request: *mut ngx_http_request_t,
    pub current_request: *mut ngx_http_request_t,
}
impl ::std::default::Default for Struct_ngx_http_log_ctx_s {
    fn default() -> Struct_ngx_http_log_ctx_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_ngx_http_chunked_s {
    pub state: ngx_uint_t,
    pub size: off_t,
    pub length: off_t,
}
impl ::std::default::Default for Struct_ngx_http_chunked_s {
    fn default() -> Struct_ngx_http_chunked_s {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed239 {
    pub http_version: ngx_uint_t,
    pub code: ngx_uint_t,
    pub count: ngx_uint_t,
    pub start: *mut u_char,
    pub end: *mut u_char,
}
impl ::std::default::Default for Struct_Unnamed239 {
    fn default() -> Struct_Unnamed239 { unsafe { ::std::mem::zeroed() } }
}
pub type ngx_http_status_t = Struct_Unnamed239;

extern "C" {
   pub static mut ngx_argc: ::libc::c_int;
   pub static mut ngx_argv: *mut *mut ::libc::c_char;
   pub static mut ngx_os_argv: *mut *mut ::libc::c_char;
   pub static mut ngx_pid: ngx_pid_t;
   pub static mut ngx_channel: ngx_socket_t;
   pub static mut ngx_process_slot: ngx_int_t;
   pub static mut ngx_last_process: ngx_int_t;
   pub static mut ngx_processes: [ngx_process_t; 1024usize];
   pub static mut ngx_errlog_module: ngx_module_t;
   pub static mut ngx_use_stderr: ngx_uint_t;
   pub static mut ngx_pagesize: ngx_uint_t;
   pub static mut ngx_pagesize_shift: ngx_uint_t;
   pub static mut ngx_cacheline_size: ngx_uint_t;
   pub static mut ngx_temp_number: *mut ngx_atomic_t;
   pub static mut ngx_random_number: ngx_atomic_int_t;
   pub static mut ngx_crc32_table_short: *mut uint32_t;
   pub static mut ngx_crc32_table256: *mut uint32_t;

   pub static mut ngx_cached_time: *mut ngx_time_t;
   pub static mut ngx_cached_err_log_time: ngx_str_t;
   pub static mut ngx_cached_http_time: ngx_str_t;
   pub static mut ngx_cached_http_log_time: ngx_str_t;
   pub static mut ngx_cached_http_log_iso8601: ngx_str_t;
   pub static mut ngx_cached_syslog_time: ngx_str_t;
   pub static mut ngx_current_msec: ngx_msec_t;
   pub static mut ngx_cycle: *mut ngx_cycle_t;
   pub static mut ngx_old_cycles: ngx_array_t;
   pub static mut ngx_core_module: ngx_module_t;
   pub static mut ngx_test_config: ngx_uint_t;
   pub static mut ngx_quiet_mode: ngx_uint_t;

   pub static mut ngx_ssl_connection_index: ::libc::c_int;
   pub static mut ngx_ssl_server_conf_index: ::libc::c_int;
   pub static mut ngx_ssl_session_cache_index: ::libc::c_int;
   pub static mut ngx_ssl_session_ticket_keys_index: ::libc::c_int;
   pub static mut ngx_ssl_certificate_index: ::libc::c_int;
   pub static mut ngx_ssl_stapling_index: ::libc::c_int;
   pub static mut ngx_process: ngx_uint_t;
   pub static mut ngx_new_binary: ngx_pid_t;
   pub static mut ngx_inherited: ngx_uint_t;
   pub static mut ngx_daemonized: ngx_uint_t;
   pub static mut ngx_threaded: ngx_uint_t;
   pub static mut ngx_exiting: ngx_uint_t;
   pub static mut ngx_reap: sig_atomic_t;
   pub static mut ngx_sigio: sig_atomic_t;
   pub static mut ngx_sigalrm: sig_atomic_t;
   pub static mut ngx_quit: sig_atomic_t;
   pub static mut ngx_debug_quit: sig_atomic_t;
   pub static mut ngx_terminate: sig_atomic_t;
   pub static mut ngx_noaccept: sig_atomic_t;
   pub static mut ngx_reconfigure: sig_atomic_t;
   pub static mut ngx_reopen: sig_atomic_t;
   pub static mut ngx_change_binary: sig_atomic_t;
   pub static mut ngx_max_module: ngx_uint_t;
   pub static mut ngx_modules: *mut *mut ngx_module_t;
   pub static mut ngx_os_io: ngx_os_io_t;
   pub static mut ngx_ncpu: ngx_int_t;
   pub static mut ngx_max_sockets: ngx_int_t;
   pub static mut ngx_inherited_nonblocking: ngx_uint_t;
   pub static mut ngx_tcp_nodelay_and_tcp_nopush: ngx_uint_t;
   pub static mut ngx_linux_rtsig_max: ::libc::c_int;
   pub static mut ngx_http_variable_null_value: ngx_http_variable_value_t;
   pub static mut ngx_http_variable_true_value: ngx_http_variable_value_t;
   pub static mut ngx_http_headers_in: *mut ngx_http_header_t;
   pub static mut ngx_http_headers_out: *mut ngx_http_header_out_t;
   pub static mut ngx_event_actions: ngx_event_actions_t;
   pub static mut ngx_io: ngx_os_io_t;
   pub static mut ngx_connection_counter: *mut ngx_atomic_t;
   pub static mut ngx_accept_mutex_ptr: *mut ngx_atomic_t;
   pub static mut ngx_accept_mutex: ngx_shmtx_t;
   pub static mut ngx_use_accept_mutex: ngx_uint_t;
   pub static mut ngx_accept_events: ngx_uint_t;
   pub static mut ngx_accept_mutex_held: ngx_uint_t;
   pub static mut ngx_accept_mutex_delay: ngx_msec_t;
   pub static mut ngx_accept_disabled: ngx_int_t;
   pub static mut ngx_event_timer_alarm: sig_atomic_t;
   pub static mut ngx_event_flags: ngx_uint_t;
   pub static mut ngx_events_module: ngx_module_t;
   pub static mut ngx_event_core_module: ngx_module_t;
   pub static mut ngx_event_timer_rbtree: ngx_rbtree_t;
   pub static mut ngx_posted_accept_events: ngx_queue_t;
   pub static mut ngx_posted_events: ngx_queue_t;
   pub static mut ngx_http_upstream_module: ngx_module_t;
   pub static mut ngx_http_upstream_cache_method_mask:
              *mut ngx_conf_bitmask_t;
   pub static mut ngx_http_upstream_ignore_headers_masks:
              *mut ngx_conf_bitmask_t;
   pub static mut ngx_http_core_module: ngx_module_t;
   pub static mut ngx_http_max_module: ngx_uint_t;
   pub static mut ngx_http_core_get_method: ngx_str_t;
   pub static mut ngx_http_cache_status: *mut ngx_str_t;
   pub static mut ngx_http_ssi_filter_module: ngx_module_t;
   pub static mut ngx_http_ssl_module: ngx_module_t;
   pub static mut ngx_http_module: ngx_module_t;
   pub static mut ngx_http_html_default_types: *mut ngx_str_t;
   pub static mut ngx_http_top_header_filter:
              ngx_http_output_header_filter_pt;
   pub static mut ngx_http_top_body_filter: ngx_http_output_body_filter_pt;
}


extern "C" {
   pub fn ngx_strerror(err: ngx_err_t, errstr: *mut u_char, size: size_t)
    -> *mut u_char;
   pub fn ngx_strerror_init() -> ngx_int_t;
   pub fn ngx_spinlock(lock: *mut ngx_atomic_t, value: ngx_atomic_int_t,
                       spin: ngx_uint_t) -> ();
   pub fn ngx_rbtree_insert(tree: *mut ngx_rbtree_t,
                           node: *mut ngx_rbtree_node_t) -> ();
   pub fn ngx_rbtree_delete(tree: *mut ngx_rbtree_t,
                           node: *mut ngx_rbtree_node_t) -> ();
   pub fn ngx_rbtree_insert_value(root: *mut ngx_rbtree_node_t,
                                  node: *mut ngx_rbtree_node_t,
                                  sentinel: *mut ngx_rbtree_node_t) -> ();
   pub fn ngx_rbtree_insert_timer_value(root: *mut ngx_rbtree_node_t,
                                        node: *mut ngx_rbtree_node_t,
                                        sentinel: *mut ngx_rbtree_node_t)
    -> ();
   pub fn ngx_timezone_update() -> ();
   pub fn ngx_localtime(s: time_t, tm: *mut ngx_tm_t) -> ();
   pub fn ngx_libc_localtime(s: time_t, tm: *mut Struct_tm) -> ();
   pub fn ngx_libc_gmtime(s: time_t, tm: *mut Struct_tm) -> ();
   pub fn ngx_nonblocking(s: ngx_socket_t) -> ::libc::c_int;
   pub fn ngx_blocking(s: ngx_socket_t) -> ::libc::c_int;
   pub fn ngx_tcp_nopush(s: ngx_socket_t) -> ::libc::c_int;
   pub fn ngx_tcp_push(s: ngx_socket_t) -> ::libc::c_int;
   pub fn ngx_strlow(dst: *mut u_char, src: *mut u_char, n: size_t) -> ();
   pub fn ngx_cpystrn(dst: *mut u_char, src: *mut u_char, n: size_t)
    -> *mut u_char;
   pub fn ngx_pstrdup(pool: *mut ngx_pool_t, src: *mut ngx_str_t)
    -> *mut u_char;
   pub fn ngx_sprintf(buf: *mut u_char, fmt: *const ::libc::c_char, ...)
    -> *mut u_char;
   pub fn ngx_snprintf(buf: *mut u_char, max: size_t,
                       fmt: *const ::libc::c_char, ...) -> *mut u_char;
   pub fn ngx_slprintf(buf: *mut u_char, last: *mut u_char,
                       fmt: *const ::libc::c_char, ...) -> *mut u_char;
   pub fn ngx_vslprintf(buf: *mut u_char, last: *mut u_char,
                        fmt: *const ::libc::c_char, args: va_list)
    -> *mut u_char;
   pub fn ngx_strcasecmp(s1: *mut u_char, s2: *mut u_char) -> ngx_int_t;
   pub fn ngx_strncasecmp(s1: *mut u_char, s2: *mut u_char, n: size_t)
    -> ngx_int_t;
   pub fn ngx_strnstr(s1: *mut u_char, s2: *mut ::libc::c_char, n: size_t)
    -> *mut u_char;
   pub fn ngx_strstrn(s1: *mut u_char, s2: *mut ::libc::c_char, n: size_t)
    -> *mut u_char;
   pub fn ngx_strcasestrn(s1: *mut u_char, s2: *mut ::libc::c_char,
                          n: size_t) -> *mut u_char;
   pub fn ngx_strlcasestrn(s1: *mut u_char, last: *mut u_char,
                           s2: *mut u_char, n: size_t) -> *mut u_char;
   pub fn ngx_rstrncmp(s1: *mut u_char, s2: *mut u_char, n: size_t)
    -> ngx_int_t;
   pub fn ngx_rstrncasecmp(s1: *mut u_char, s2: *mut u_char, n: size_t)
    -> ngx_int_t;
   pub fn ngx_memn2cmp(s1: *mut u_char, s2: *mut u_char, n1: size_t,
                       n2: size_t) -> ngx_int_t;
   pub fn ngx_dns_strcmp(s1: *mut u_char, s2: *mut u_char) -> ngx_int_t;
   pub fn ngx_filename_cmp(s1: *mut u_char, s2: *mut u_char, n: size_t)
    -> ngx_int_t;
   pub fn ngx_atoi(line: *mut u_char, n: size_t) -> ngx_int_t;
   pub fn ngx_atofp(line: *mut u_char, n: size_t, point: size_t)
    -> ngx_int_t;
   pub fn ngx_atosz(line: *mut u_char, n: size_t) -> ssize_t;
   pub fn ngx_atoof(line: *mut u_char, n: size_t) -> off_t;
   pub fn ngx_atotm(line: *mut u_char, n: size_t) -> time_t;
   pub fn ngx_hextoi(line: *mut u_char, n: size_t) -> ngx_int_t;
   pub fn ngx_hex_dump(dst: *mut u_char, src: *mut u_char, len: size_t)
    -> *mut u_char;
   pub fn ngx_encode_base64(dst: *mut ngx_str_t, src: *mut ngx_str_t) -> ();
   pub fn ngx_encode_base64url(dst: *mut ngx_str_t, src: *mut ngx_str_t)
    -> ();
   pub fn ngx_decode_base64(dst: *mut ngx_str_t, src: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_decode_base64url(dst: *mut ngx_str_t, src: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_utf8_decode(p: *mut *mut u_char, n: size_t) -> uint32_t;
   pub fn ngx_utf8_length(p: *mut u_char, n: size_t) -> size_t;
   pub fn ngx_utf8_cpystrn(dst: *mut u_char, src: *mut u_char, n: size_t,
                           len: size_t) -> *mut u_char;
   pub fn ngx_escape_uri(dst: *mut u_char, src: *mut u_char, size: size_t,
                        _type: ngx_uint_t) -> uintptr_t;
   pub fn ngx_unescape_uri(dst: *mut *mut u_char, src: *mut *mut u_char,
                           size: size_t, _type: ngx_uint_t) -> ();
   pub fn ngx_escape_html(dst: *mut u_char, src: *mut u_char, size: size_t)
    -> uintptr_t;
   pub fn ngx_escape_json(dst: *mut u_char, src: *mut u_char, size: size_t)
    -> uintptr_t;
   pub fn ngx_str_rbtree_insert_value(temp: *mut ngx_rbtree_node_t,
                                      node: *mut ngx_rbtree_node_t,
                                      sentinel: *mut ngx_rbtree_node_t)
    -> ();
   pub fn ngx_str_rbtree_lookup(rbtree: *mut ngx_rbtree_t,
                                name: *mut ngx_str_t, hash: uint32_t)
    -> *mut ngx_str_node_t;
   pub fn ngx_sort(base: *mut ::libc::c_void, n: size_t, size: size_t,
                   cmp:
                       ::std::option::Option<extern "C" fn
                                                 (arg1:
                                                      *const ::libc::c_void,
                                                  arg2:
                                                      *const ::libc::c_void)
                                                 -> ngx_int_t>) -> ();
   pub fn ngx_open_tempfile(name: *mut u_char, persistent: ngx_uint_t,
                           access: ngx_uint_t) -> ngx_fd_t;
   pub fn ngx_read_file(file: *mut ngx_file_t, buf: *mut u_char,
                        size: size_t, offset: off_t) -> ssize_t;
   pub fn ngx_write_file(file: *mut ngx_file_t, buf: *mut u_char,
                        size: size_t, offset: off_t) -> ssize_t;
   pub fn ngx_write_chain_to_file(file: *mut ngx_file_t,
                                  ce: *mut ngx_chain_t, offset: off_t,
                                  pool: *mut ngx_pool_t) -> ssize_t;
   pub fn ngx_set_file_time(name: *mut u_char, fd: ngx_fd_t, s: time_t)
    -> ngx_int_t;
   pub fn ngx_create_file_mapping(fm: *mut ngx_file_mapping_t) -> ngx_int_t;
   pub fn ngx_close_file_mapping(fm: *mut ngx_file_mapping_t) -> ();
   pub fn ngx_open_dir(name: *mut ngx_str_t, dir: *mut ngx_dir_t)
    -> ngx_int_t;
   pub fn ngx_read_dir(dir: *mut ngx_dir_t) -> ngx_int_t;
   pub fn ngx_open_glob(gl: *mut ngx_glob_t) -> ngx_int_t;
   pub fn ngx_read_glob(gl: *mut ngx_glob_t, name: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_close_glob(gl: *mut ngx_glob_t) -> ();
   pub fn ngx_trylock_fd(fd: ngx_fd_t) -> ngx_err_t;
   pub fn ngx_lock_fd(fd: ngx_fd_t) -> ngx_err_t;
   pub fn ngx_unlock_fd(fd: ngx_fd_t) -> ngx_err_t;
   pub fn ngx_read_ahead(fd: ngx_fd_t, n: size_t) -> ngx_int_t;
   pub fn ngx_directio_on(fd: ngx_fd_t) -> ngx_int_t;
   pub fn ngx_directio_off(fd: ngx_fd_t) -> ngx_int_t;
   pub fn ngx_fs_bsize(name: *mut u_char) -> size_t;
   pub fn ngx_shm_alloc(shm: *mut ngx_shm_t) -> ngx_int_t;
   pub fn ngx_shm_free(shm: *mut ngx_shm_t) -> ();
   pub fn ngx_setaffinity(cpu_affinity: uint64_t, log: *mut ngx_log_t) -> ();
   pub fn ngx_init_setproctitle(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_setproctitle(title: *mut ::libc::c_char) -> ();
   pub fn ngx_spawn_process(cycle: *mut ngx_cycle_t,
                           _proc: ngx_spawn_proc_pt,
                           data: *mut ::libc::c_void,
                           name: *mut ::libc::c_char, respawn: ngx_int_t)
    -> ngx_pid_t;
   pub fn ngx_execute(cycle: *mut ngx_cycle_t, ctx: *mut ngx_exec_ctx_t)
    -> ngx_pid_t;
   pub fn ngx_init_signals(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_debug_point() -> ();
   pub fn ngx_libc_crypt(pool: *mut ngx_pool_t, key: *mut u_char,
                        salt: *mut u_char, encrypted: *mut *mut u_char)
    -> ngx_int_t;
   pub fn ngx_parse_size(line: *mut ngx_str_t) -> ssize_t;
   pub fn ngx_parse_offset(line: *mut ngx_str_t) -> off_t;
   pub fn ngx_parse_time(line: *mut ngx_str_t, is_sec: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_log_error_core(level: ngx_uint_t, log: *mut ngx_log_t,
                             err: ngx_err_t, fmt: *const ::libc::c_char, ...)
    -> ();
   pub fn ngx_log_init(prefix: *mut u_char) -> *mut ngx_log_t;
   pub fn ngx_log_abort(err: ngx_err_t, fmt: *const ::libc::c_char, ...)
    -> ();
   pub fn ngx_log_stderr(err: ngx_err_t, fmt: *const ::libc::c_char, ...)
    -> ();
   pub fn ngx_log_errno(buf: *mut u_char, last: *mut u_char, err: ngx_err_t)
    -> *mut u_char;
   pub fn ngx_log_open_default(cycle: *mut ngx_cycle_t) -> ngx_int_t;
   pub fn ngx_log_redirect_stderr(cycle: *mut ngx_cycle_t) -> ngx_int_t;
   pub fn ngx_log_get_file_log(head: *mut ngx_log_t) -> *mut ngx_log_t;
   pub fn ngx_log_set_log(cf: *mut ngx_conf_t, head: *mut *mut ngx_log_t)
    -> *mut ::libc::c_char;
   pub fn ngx_alloc(size: size_t, log: *mut ngx_log_t)
    -> *mut ::libc::c_void;
   pub fn ngx_calloc(size: size_t, log: *mut ngx_log_t)
    -> *mut ::libc::c_void;
   pub fn ngx_memalign(alignment: size_t, size: size_t, log: *mut ngx_log_t)
    -> *mut ::libc::c_void;
   pub fn ngx_create_pool(size: size_t, log: *mut ngx_log_t)
    -> *mut ngx_pool_t;
   pub fn ngx_destroy_pool(pool: *mut ngx_pool_t) -> ();
   pub fn ngx_reset_pool(pool: *mut ngx_pool_t) -> ();
   pub fn ngx_palloc(pool: *mut ngx_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_pnalloc(pool: *mut ngx_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_pcalloc(pool: *mut ngx_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_pmemalign(pool: *mut ngx_pool_t, size: size_t,
                        alignment: size_t) -> *mut ::libc::c_void;
   pub fn ngx_pfree(pool: *mut ngx_pool_t, p: *mut ::libc::c_void)
    -> ngx_int_t;
   pub fn ngx_pool_cleanup_add(p: *mut ngx_pool_t, size: size_t)
    -> *mut ngx_pool_cleanup_t;
   pub fn ngx_pool_run_cleanup_file(p: *mut ngx_pool_t, fd: ngx_fd_t) -> ();
   pub fn ngx_pool_cleanup_file(data: *mut ::libc::c_void) -> ();
   pub fn ngx_pool_delete_file(data: *mut ::libc::c_void) -> ();
   pub fn ngx_create_temp_buf(pool: *mut ngx_pool_t, size: size_t)
    -> *mut ngx_buf_t;
   pub fn ngx_create_chain_of_bufs(pool: *mut ngx_pool_t,
                                   bufs: *mut ngx_bufs_t)
    -> *mut ngx_chain_t;
   pub fn ngx_alloc_chain_link(pool: *mut ngx_pool_t) -> *mut ngx_chain_t;
   pub fn ngx_output_chain(ctx: *mut ngx_output_chain_ctx_t,
                           _in: *mut ngx_chain_t) -> ngx_int_t;
   pub fn ngx_chain_writer(ctx: *mut ::libc::c_void, _in: *mut ngx_chain_t)
    -> ngx_int_t;
   pub fn ngx_chain_add_copy(pool: *mut ngx_pool_t,
                             chain: *mut *mut ngx_chain_t,
                             _in: *mut ngx_chain_t) -> ngx_int_t;
   pub fn ngx_chain_get_free_buf(p: *mut ngx_pool_t,
                                 free: *mut *mut ngx_chain_t)
    -> *mut ngx_chain_t;
   pub fn ngx_chain_update_chains(p: *mut ngx_pool_t,
                                  free: *mut *mut ngx_chain_t,
                                  busy: *mut *mut ngx_chain_t,
                                  out: *mut *mut ngx_chain_t,
                                  tag: ngx_buf_tag_t) -> ();
   pub fn ngx_chain_coalesce_file(_in: *mut *mut ngx_chain_t, limit: off_t)
    -> off_t;
   pub fn ngx_chain_update_sent(_in: *mut ngx_chain_t, sent: off_t)
    -> *mut ngx_chain_t;
   pub fn ngx_queue_middle(queue: *mut ngx_queue_t) -> *mut ngx_queue_t;
   pub fn ngx_queue_sort(queue: *mut ngx_queue_t,
                        cmp:
                             ::std::option::Option<extern "C" fn
                                                       (arg1:
                                                            *const ngx_queue_t,
                                                        arg2:
                                                            *const ngx_queue_t)
                                                       -> ngx_int_t>) -> ();
   pub fn ngx_array_create(p: *mut ngx_pool_t, n: ngx_uint_t, size: size_t)
    -> *mut ngx_array_t;
   pub fn ngx_array_destroy(a: *mut ngx_array_t) -> ();
   pub fn ngx_array_push(a: *mut ngx_array_t) -> *mut ::libc::c_void;
   pub fn ngx_array_push_n(a: *mut ngx_array_t, n: ngx_uint_t)
    -> *mut ::libc::c_void;
   pub fn ngx_list_create(pool: *mut ngx_pool_t, n: ngx_uint_t, size: size_t)
    -> *mut ngx_list_t;
   pub fn ngx_list_push(list: *mut ngx_list_t) -> *mut ::libc::c_void;
   pub fn ngx_hash_find(hash: *mut ngx_hash_t, key: ngx_uint_t,
                        name: *mut u_char, len: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_hash_find_wc_head(hwc: *mut ngx_hash_wildcard_t,
                                name: *mut u_char, len: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_hash_find_wc_tail(hwc: *mut ngx_hash_wildcard_t,
                                name: *mut u_char, len: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_hash_find_combined(hash: *mut ngx_hash_combined_t,
                                 key: ngx_uint_t, name: *mut u_char,
                                 len: size_t) -> *mut ::libc::c_void;
   pub fn ngx_hash_init(hinit: *mut ngx_hash_init_t,
                        names: *mut ngx_hash_key_t, nelts: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_hash_wildcard_init(hinit: *mut ngx_hash_init_t,
                                 names: *mut ngx_hash_key_t,
                                 nelts: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_hash_key(data: *mut u_char, len: size_t) -> ngx_uint_t;
   pub fn ngx_hash_key_lc(data: *mut u_char, len: size_t) -> ngx_uint_t;
   pub fn ngx_hash_strlow(dst: *mut u_char, src: *mut u_char, n: size_t)
    -> ngx_uint_t;
   pub fn ngx_hash_keys_array_init(ha: *mut ngx_hash_keys_arrays_t,
                                   _type: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_hash_add_key(ha: *mut ngx_hash_keys_arrays_t,
                           key: *mut ngx_str_t, value: *mut ::libc::c_void,
                           flags: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_get_full_name(pool: *mut ngx_pool_t, prefix: *mut ngx_str_t,
                           name: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_write_chain_to_temp_file(tf: *mut ngx_temp_file_t,
                                       chain: *mut ngx_chain_t) -> ssize_t;
   pub fn ngx_create_temp_file(file: *mut ngx_file_t, path: *mut ngx_path_t,
                              pool: *mut ngx_pool_t, persistent: ngx_uint_t,
                              clean: ngx_uint_t, access: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_create_hashed_filename(path: *mut ngx_path_t,
                                     file: *mut u_char, len: size_t) -> ();
   pub fn ngx_create_path(file: *mut ngx_file_t, path: *mut ngx_path_t)
    -> ngx_int_t;
   pub fn ngx_create_full_path(dir: *mut u_char, access: ngx_uint_t)
    -> ngx_err_t;
   pub fn ngx_add_path(cf: *mut ngx_conf_t, slot: *mut *mut ngx_path_t)
    -> ngx_int_t;
   pub fn ngx_create_paths(cycle: *mut ngx_cycle_t, user: ngx_uid_t)
    -> ngx_int_t;
   pub fn ngx_ext_rename_file(src: *mut ngx_str_t, to: *mut ngx_str_t,
                              ext: *mut ngx_ext_rename_file_t) -> ngx_int_t;
   pub fn ngx_copy_file(from: *mut u_char, to: *mut u_char,
                        cf: *mut ngx_copy_file_t) -> ngx_int_t;
   pub fn ngx_walk_tree(ctx: *mut ngx_tree_ctx_t, tree: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_next_temp_number(collision: ngx_uint_t) -> ngx_atomic_uint_t;
   pub fn ngx_conf_set_path_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_merge_path_value(cf: *mut ngx_conf_t,
                                    path: *mut *mut ngx_path_t,
                                    prev: *mut ngx_path_t,
                                    init: *mut ngx_path_init_t)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_access_slot(cf: *mut ngx_conf_t,
                                   cmd: *mut ngx_command_t,
                                   conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_crc32_table_init() -> ngx_int_t;
   pub fn ngx_murmur_hash2(data: *mut u_char, len: size_t) -> uint32_t;

   pub fn ngx_regex_init() -> ();
   pub fn ngx_regex_compile(rc: *mut ngx_regex_compile_t) -> ngx_int_t;
   pub fn ngx_regex_exec_array(a: *mut ngx_array_t, s: *mut ngx_str_t,
                              log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_radix_tree_create(pool: *mut ngx_pool_t,
                                preallocate: ngx_int_t)
    -> *mut ngx_radix_tree_t;
   pub fn ngx_radix32tree_insert(tree: *mut ngx_radix_tree_t, key: uint32_t,
                                 mask: uint32_t, value: uintptr_t)
    -> ngx_int_t;
   pub fn ngx_radix32tree_delete(tree: *mut ngx_radix_tree_t, key: uint32_t,
                                 mask: uint32_t) -> ngx_int_t;
   pub fn ngx_radix32tree_find(tree: *mut ngx_radix_tree_t, key: uint32_t)
    -> uintptr_t;
   pub fn ngx_time_init() -> ();
   pub fn ngx_time_update() -> ();
   pub fn ngx_time_sigsafe_update() -> ();
   pub fn ngx_http_time(buf: *mut u_char, t: time_t) -> *mut u_char;
   pub fn ngx_http_cookie_time(buf: *mut u_char, t: time_t) -> *mut u_char;
   pub fn ngx_gmtime(t: time_t, tp: *mut ngx_tm_t) -> ();
   pub fn ngx_next_time(when: time_t) -> time_t;
   pub fn ngx_shmtx_create(mtx: *mut ngx_shmtx_t, addr: *mut ngx_shmtx_sh_t,
                           name: *mut u_char) -> ngx_int_t;
   pub fn ngx_shmtx_destroy(mtx: *mut ngx_shmtx_t) -> ();
   pub fn ngx_shmtx_trylock(mtx: *mut ngx_shmtx_t) -> ngx_uint_t;
   pub fn ngx_shmtx_lock(mtx: *mut ngx_shmtx_t) -> ();
   pub fn ngx_shmtx_unlock(mtx: *mut ngx_shmtx_t) -> ();
   pub fn ngx_shmtx_force_unlock(mtx: *mut ngx_shmtx_t, pid: ngx_pid_t)
    -> ngx_uint_t;
   pub fn ngx_slab_init(pool: *mut ngx_slab_pool_t) -> ();
   pub fn ngx_slab_alloc(pool: *mut ngx_slab_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_slab_alloc_locked(pool: *mut ngx_slab_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_slab_calloc(pool: *mut ngx_slab_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_slab_calloc_locked(pool: *mut ngx_slab_pool_t, size: size_t)
    -> *mut ::libc::c_void;
   pub fn ngx_slab_free(pool: *mut ngx_slab_pool_t, p: *mut ::libc::c_void)
    -> ();
   pub fn ngx_slab_free_locked(pool: *mut ngx_slab_pool_t,
                              p: *mut ::libc::c_void) -> ();
   pub fn ngx_inet_addr(text: *mut u_char, len: size_t) -> in_addr_t;
   pub fn ngx_sock_ntop(sa: *mut Struct_sockaddr, socklen: socklen_t,
                        text: *mut u_char, len: size_t, port: ngx_uint_t)
    -> size_t;
   pub fn ngx_inet_ntop(family: ::libc::c_int, addr: *mut ::libc::c_void,
                        text: *mut u_char, len: size_t) -> size_t;
   pub fn ngx_ptocidr(text: *mut ngx_str_t, cidr: *mut ngx_cidr_t)
    -> ngx_int_t;
   pub fn ngx_parse_addr(pool: *mut ngx_pool_t, addr: *mut ngx_addr_t,
                        text: *mut u_char, len: size_t) -> ngx_int_t;
   pub fn ngx_parse_url(pool: *mut ngx_pool_t, u: *mut ngx_url_t)
    -> ngx_int_t;
   pub fn ngx_inet_resolve_host(pool: *mut ngx_pool_t, u: *mut ngx_url_t)
    -> ngx_int_t;
   pub fn ngx_cmp_sockaddr(sa1: *mut Struct_sockaddr, slen1: socklen_t,
                           sa2: *mut Struct_sockaddr, slen2: socklen_t,
                           cmp_port: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_init_cycle(old_cycle: *mut ngx_cycle_t) -> *mut ngx_cycle_t;
   pub fn ngx_create_pidfile(name: *mut ngx_str_t, log: *mut ngx_log_t)
    -> ngx_int_t;
   pub fn ngx_delete_pidfile(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_signal_process(cycle: *mut ngx_cycle_t,
                             sig: *mut ::libc::c_char) -> ngx_int_t;
   pub fn ngx_reopen_files(cycle: *mut ngx_cycle_t, user: ngx_uid_t) -> ();
   pub fn ngx_set_environment(cycle: *mut ngx_cycle_t, last: *mut ngx_uint_t)
    -> *mut *mut ::libc::c_char;
   pub fn ngx_exec_new_binary(cycle: *mut ngx_cycle_t,
                              argv: *const *mut ::libc::c_char) -> ngx_pid_t;
   pub fn ngx_get_cpu_affinity(n: ngx_uint_t) -> uint64_t;
   pub fn ngx_shared_memory_add(cf: *mut ngx_conf_t, name: *mut ngx_str_t,
                                size: size_t, tag: *mut ::libc::c_void)
    -> *mut ngx_shm_zone_t;
   pub fn ngx_resolver_create(cf: *mut ngx_conf_t, names: *mut ngx_str_t,
                              n: ngx_uint_t) -> *mut ngx_resolver_t;
   pub fn ngx_resolve_start(r: *mut ngx_resolver_t,
                           temp: *mut ngx_resolver_ctx_t)
    -> *mut ngx_resolver_ctx_t;
   pub fn ngx_resolve_name(ctx: *mut ngx_resolver_ctx_t) -> ngx_int_t;
   pub fn ngx_resolve_name_done(ctx: *mut ngx_resolver_ctx_t) -> ();
   pub fn ngx_resolve_addr(ctx: *mut ngx_resolver_ctx_t) -> ngx_int_t;
   pub fn ngx_resolve_addr_done(ctx: *mut ngx_resolver_ctx_t) -> ();
   pub fn ngx_resolver_strerror(err: ngx_int_t) -> *mut ::libc::c_char;

   pub fn ngx_ssl_init(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_ssl_create(ssl: *mut ngx_ssl_t, protocols: ngx_uint_t,
                        data: *mut ::libc::c_void) -> ngx_int_t;
   pub fn ngx_ssl_certificate(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                              cert: *mut ngx_str_t, key: *mut ngx_str_t,
                              passwords: *mut ngx_array_t) -> ngx_int_t;
   pub fn ngx_ssl_client_certificate(cf: *mut ngx_conf_t,
                                     ssl: *mut ngx_ssl_t,
                                     cert: *mut ngx_str_t, depth: ngx_int_t)
    -> ngx_int_t;
   pub fn ngx_ssl_trusted_certificate(cf: *mut ngx_conf_t,
                                      ssl: *mut ngx_ssl_t,
                                      cert: *mut ngx_str_t, depth: ngx_int_t)
    -> ngx_int_t;
   pub fn ngx_ssl_crl(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                      crl: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_ssl_stapling(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                           file: *mut ngx_str_t, responder: *mut ngx_str_t,
                           verify: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_ssl_stapling_resolver(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                                    resolver: *mut ngx_resolver_t,
                                    resolver_timeout: ngx_msec_t)
    -> ngx_int_t;
   pub fn ngx_ssl_rsa512_key_callback(ssl_conn: *mut SSL,
                                      is_export: ::libc::c_int,
                                      key_length: ::libc::c_int) -> *mut RSA;
   pub fn ngx_ssl_read_password_file(cf: *mut ngx_conf_t,
                                     file: *mut ngx_str_t)
    -> *mut ngx_array_t;
   pub fn ngx_ssl_dhparam(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                          file: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_ssl_ecdh_curve(cf: *mut ngx_conf_t, ssl: *mut ngx_ssl_t,
                             name: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_ssl_session_cache(ssl: *mut ngx_ssl_t,
                                sess_ctx: *mut ngx_str_t,
                                builtin_session_cache: ssize_t,
                                shm_zone: *mut ngx_shm_zone_t,
                                timeout: time_t) -> ngx_int_t;
   pub fn ngx_ssl_session_ticket_keys(cf: *mut ngx_conf_t,
                                      ssl: *mut ngx_ssl_t,
                                      paths: *mut ngx_array_t) -> ngx_int_t;
   pub fn ngx_ssl_session_cache_init(shm_zone: *mut ngx_shm_zone_t,
                                     data: *mut ::libc::c_void) -> ngx_int_t;
   pub fn ngx_ssl_create_connection(ssl: *mut ngx_ssl_t,
                                    c: *mut ngx_connection_t,
                                    flags: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_ssl_remove_cached_session(ssl: *mut SSL_CTX,
                                        sess: *mut SSL_SESSION) -> ();
   pub fn ngx_ssl_set_session(c: *mut ngx_connection_t,
                              session: *mut SSL_SESSION) -> ngx_int_t;
   pub fn ngx_ssl_check_host(c: *mut ngx_connection_t, name: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_protocol(c: *mut ngx_connection_t,
                              pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_cipher_name(c: *mut ngx_connection_t,
                                  pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_session_id(c: *mut ngx_connection_t,
                                 pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_session_reused(c: *mut ngx_connection_t,
                                     pool: *mut ngx_pool_t,
                                     s: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_ssl_get_server_name(c: *mut ngx_connection_t,
                                  pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_raw_certificate(c: *mut ngx_connection_t,
                                      pool: *mut ngx_pool_t,
                                      s: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_ssl_get_certificate(c: *mut ngx_connection_t,
                                  pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_subject_dn(c: *mut ngx_connection_t,
                                 pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_issuer_dn(c: *mut ngx_connection_t,
                                pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_serial_number(c: *mut ngx_connection_t,
                                    pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_fingerprint(c: *mut ngx_connection_t,
                                  pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_get_client_verify(c: *mut ngx_connection_t,
                                    pool: *mut ngx_pool_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_ssl_handshake(c: *mut ngx_connection_t) -> ngx_int_t;
   pub fn ngx_ssl_recv(c: *mut ngx_connection_t, buf: *mut u_char,
                       size: size_t) -> ssize_t;
   pub fn ngx_ssl_write(c: *mut ngx_connection_t, data: *mut u_char,
                        size: size_t) -> ssize_t;
   pub fn ngx_ssl_recv_chain(c: *mut ngx_connection_t, cl: *mut ngx_chain_t,
                             limit: off_t) -> ssize_t;
   pub fn ngx_ssl_send_chain(c: *mut ngx_connection_t, _in: *mut ngx_chain_t,
                             limit: off_t) -> *mut ngx_chain_t;
   pub fn ngx_ssl_free_buffer(c: *mut ngx_connection_t) -> ();
   pub fn ngx_ssl_shutdown(c: *mut ngx_connection_t) -> ngx_int_t;
   pub fn ngx_ssl_error(level: ngx_uint_t, log: *mut ngx_log_t,
                        err: ngx_err_t, fmt: *mut ::libc::c_char, ...) -> ();
   pub fn ngx_ssl_cleanup_ctx(data: *mut ::libc::c_void) -> ();
   pub fn ngx_master_process_cycle(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_single_process_cycle(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_conf_deprecated(cf: *mut ngx_conf_t, post: *mut ::libc::c_void,
                              data: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_check_num_bounds(cf: *mut ngx_conf_t,
                                    post: *mut ::libc::c_void,
                                    data: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_param(cf: *mut ngx_conf_t) -> *mut ::libc::c_char;
   pub fn ngx_conf_parse(cf: *mut ngx_conf_t, filename: *mut ngx_str_t)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_include(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                           conf: *mut ::libc::c_void) -> *mut ::libc::c_char;
   pub fn ngx_conf_full_name(cycle: *mut ngx_cycle_t, name: *mut ngx_str_t,
                             conf_prefix: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_conf_open_file(cycle: *mut ngx_cycle_t, name: *mut ngx_str_t)
    -> *mut ngx_open_file_t;
   pub fn ngx_conf_log_error(level: ngx_uint_t, cf: *mut ngx_conf_t,
                             err: ngx_err_t, fmt: *const ::libc::c_char, ...)
    -> ();
   pub fn ngx_conf_set_flag_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_str_slot(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                                conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_str_array_slot(cf: *mut ngx_conf_t,
                                      cmd: *mut ngx_command_t,
                                      conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_keyval_slot(cf: *mut ngx_conf_t,
                                   cmd: *mut ngx_command_t,
                                   conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_num_slot(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                                conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_size_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_off_slot(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                                conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_msec_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_sec_slot(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                                conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_bufs_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_enum_slot(cf: *mut ngx_conf_t,
                                 cmd: *mut ngx_command_t,
                                 conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_conf_set_bitmask_slot(cf: *mut ngx_conf_t,
                                    cmd: *mut ngx_command_t,
                                    conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_open_file_cache_init(pool: *mut ngx_pool_t, max: ngx_uint_t,
                                   inactive: time_t)
    -> *mut ngx_open_file_cache_t;
   pub fn ngx_open_cached_file(cache: *mut ngx_open_file_cache_t,
                              name: *mut ngx_str_t,
                              of: *mut ngx_open_file_info_t,
                              pool: *mut ngx_pool_t) -> ngx_int_t;
   pub fn ngx_os_init(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_os_status(log: *mut ngx_log_t) -> ();
   pub fn ngx_os_specific_init(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_os_specific_status(log: *mut ngx_log_t) -> ();
   pub fn ngx_daemon(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_os_signal_process(cycle: *mut ngx_cycle_t,
                                sig: *mut ::libc::c_char, pid: ngx_int_t)
    -> ngx_int_t;
   pub fn ngx_unix_recv(c: *mut ngx_connection_t, buf: *mut u_char,
                        size: size_t) -> ssize_t;
   pub fn ngx_readv_chain(c: *mut ngx_connection_t, entry: *mut ngx_chain_t,
                          limit: off_t) -> ssize_t;
   pub fn ngx_udp_unix_recv(c: *mut ngx_connection_t, buf: *mut u_char,
                           size: size_t) -> ssize_t;
   pub fn ngx_unix_send(c: *mut ngx_connection_t, buf: *mut u_char,
                        size: size_t) -> ssize_t;
   pub fn ngx_writev_chain(c: *mut ngx_connection_t, _in: *mut ngx_chain_t,
                           limit: off_t) -> *mut ngx_chain_t;
   pub fn ngx_output_chain_to_iovec(vec: *mut ngx_iovec_t,
                                    _in: *mut ngx_chain_t, limit: size_t,
                                    log: *mut ngx_log_t) -> *mut ngx_chain_t;
   pub fn ngx_writev(c: *mut ngx_connection_t, vec: *mut ngx_iovec_t)
    -> ssize_t;
   pub fn ngx_linux_sendfile_chain(c: *mut ngx_connection_t,
                                   _in: *mut ngx_chain_t, limit: off_t)
    -> *mut ngx_chain_t;
   pub fn ngx_create_listening(cf: *mut ngx_conf_t,
                              sockaddr: *mut ::libc::c_void,
                              socklen: socklen_t) -> *mut ngx_listening_t;
   pub fn ngx_set_inherited_sockets(cycle: *mut ngx_cycle_t) -> ngx_int_t;
   pub fn ngx_open_listening_sockets(cycle: *mut ngx_cycle_t) -> ngx_int_t;
   pub fn ngx_configure_listening_sockets(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_close_listening_sockets(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_close_connection(c: *mut ngx_connection_t) -> ();
   pub fn ngx_connection_local_sockaddr(c: *mut ngx_connection_t,
                                        s: *mut ngx_str_t, port: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_connection_error(c: *mut ngx_connection_t, err: ngx_err_t,
                              text: *mut ::libc::c_char) -> ngx_int_t;
   pub fn ngx_get_connection(s: ngx_socket_t, log: *mut ngx_log_t)
    -> *mut ngx_connection_t;
   pub fn ngx_free_connection(c: *mut ngx_connection_t) -> ();
   pub fn ngx_reusable_connection(c: *mut ngx_connection_t,
                                  reusable: ngx_uint_t) -> ();
   pub fn ngx_syslog_process_conf(cf: *mut ngx_conf_t,
                                  peer: *mut ngx_syslog_peer_t)
    -> *mut ::libc::c_char;
   pub fn ngx_syslog_add_header(peer: *mut ngx_syslog_peer_t,
                                buf: *mut u_char) -> *mut u_char;
   pub fn ngx_syslog_writer(log: *mut ngx_log_t, level: ngx_uint_t,
                           buf: *mut u_char, len: size_t) -> ();
   pub fn ngx_syslog_send(peer: *mut ngx_syslog_peer_t, buf: *mut u_char,
                          len: size_t) -> ssize_t;
   pub fn ngx_proxy_protocol_parse(c: *mut ngx_connection_t,
                                   buf: *mut u_char, last: *mut u_char)
    -> *mut u_char;
   pub fn ngx_cpuinfo() -> ();
   pub fn ngx_http_add_variable(cf: *mut ngx_conf_t, name: *mut ngx_str_t,
                                flags: ngx_uint_t)
    -> *mut ngx_http_variable_t;
   pub fn ngx_http_get_variable_index(cf: *mut ngx_conf_t,
                                      name: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_http_get_indexed_variable(r: *mut ngx_http_request_t,
                                        index: ngx_uint_t)
    -> *mut ngx_http_variable_value_t;
   pub fn ngx_http_get_flushed_variable(r: *mut ngx_http_request_t,
                                        index: ngx_uint_t)
    -> *mut ngx_http_variable_value_t;
   pub fn ngx_http_get_variable(r: *mut ngx_http_request_t,
                                name: *mut ngx_str_t, key: ngx_uint_t)
    -> *mut ngx_http_variable_value_t;
   pub fn ngx_http_variable_unknown_header(v: *mut ngx_http_variable_value_t,
                                           var: *mut ngx_str_t,
                                           part: *mut ngx_list_part_t,
                                           prefix: size_t) -> ngx_int_t;
   pub fn ngx_http_regex_compile(cf: *mut ngx_conf_t,
                                 rc: *mut ngx_regex_compile_t)
    -> *mut ngx_http_regex_t;
   pub fn ngx_http_regex_exec(r: *mut ngx_http_request_t,
                              re: *mut ngx_http_regex_t, s: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_http_map_find(r: *mut ngx_http_request_t,
                           map: *mut ngx_http_map_t, _match: *mut ngx_str_t)
    -> *mut ::libc::c_void;
   pub fn ngx_http_variables_add_core_vars(cf: *mut ngx_conf_t) -> ngx_int_t;
   pub fn ngx_http_variables_init_vars(cf: *mut ngx_conf_t) -> ngx_int_t;
   pub fn ngx_http_script_flush_complex_value(r: *mut ngx_http_request_t,
                                              val:
                                                  *mut ngx_http_complex_value_t)
    -> ();
   pub fn ngx_http_complex_value(r: *mut ngx_http_request_t,
                                 val: *mut ngx_http_complex_value_t,
                                 value: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_http_compile_complex_value(ccv:
                                             *mut ngx_http_compile_complex_value_t)
    -> ngx_int_t;
   pub fn ngx_http_set_complex_value_slot(cf: *mut ngx_conf_t,
                                          cmd: *mut ngx_command_t,
                                          conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_test_predicates(r: *mut ngx_http_request_t,
                                   predicates: *mut ngx_array_t)
    -> ngx_int_t;
   pub fn ngx_http_set_predicate_slot(cf: *mut ngx_conf_t,
                                      cmd: *mut ngx_command_t,
                                      conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_script_variables_count(value: *mut ngx_str_t)
    -> ngx_uint_t;
   pub fn ngx_http_script_compile(sc: *mut ngx_http_script_compile_t)
    -> ngx_int_t;
   pub fn ngx_http_script_run(r: *mut ngx_http_request_t,
                              value: *mut ngx_str_t,
                              code_lengths: *mut ::libc::c_void,
                              reserved: size_t,
                              code_values: *mut ::libc::c_void)
    -> *mut u_char;
   pub fn ngx_http_script_flush_no_cacheable_variables(r:
                                                           *mut ngx_http_request_t,
                                                       indices:
                                                           *mut ngx_array_t)
    -> ();
   pub fn ngx_http_script_start_code(pool: *mut ngx_pool_t,
                                     codes: *mut *mut ngx_array_t,
                                     size: size_t) -> *mut ::libc::c_void;
   pub fn ngx_http_script_add_code(codes: *mut ngx_array_t, size: size_t,
                                   code: *mut ::libc::c_void)
    -> *mut ::libc::c_void;
   pub fn ngx_http_script_copy_len_code(e: *mut ngx_http_script_engine_t)
    -> size_t;
   pub fn ngx_http_script_copy_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_copy_var_len_code(e: *mut ngx_http_script_engine_t)
    -> size_t;
   pub fn ngx_http_script_copy_var_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_copy_capture_len_code(e:
                                                    *mut ngx_http_script_engine_t)
    -> size_t;
   pub fn ngx_http_script_copy_capture_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_mark_args_code(e: *mut ngx_http_script_engine_t)
    -> size_t;
   pub fn ngx_http_script_start_args_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_regex_start_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_regex_end_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_return_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_break_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_if_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_equal_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_not_equal_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_file_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_complex_value_code(e:
                                                 *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_value_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_set_var_code(e: *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_var_set_handler_code(e:
                                                   *mut ngx_http_script_engine_t)
    -> ();
   pub fn ngx_http_script_var_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_http_script_nop_code(e: *mut ngx_http_script_engine_t) -> ();
   pub fn ngx_event_accept(ev: *mut ngx_event_t) -> ();
   pub fn ngx_trylock_accept_mutex(cycle: *mut ngx_cycle_t) -> ngx_int_t;
   pub fn ngx_accept_log_error(log: *mut ngx_log_t, buf: *mut u_char,
                              len: size_t) -> *mut u_char;
   pub fn ngx_process_events_and_timers(cycle: *mut ngx_cycle_t) -> ();
   pub fn ngx_handle_read_event(rev: *mut ngx_event_t, flags: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_handle_write_event(wev: *mut ngx_event_t, lowat: size_t)
    -> ngx_int_t;
   pub fn ngx_send_lowat(c: *mut ngx_connection_t, lowat: size_t)
    -> ngx_int_t;
   pub fn ngx_event_timer_init(log: *mut ngx_log_t) -> ngx_int_t;
   pub fn ngx_event_find_timer() -> ngx_msec_t;
   pub fn ngx_event_expire_timers() -> ();
   pub fn ngx_event_cancel_timers() -> ();
   pub fn ngx_event_process_posted(cycle: *mut ngx_cycle_t,
                                   posted: *mut ngx_queue_t) -> ();
   pub fn ngx_event_busy_lock(bl: *mut ngx_event_busy_lock_t,
                              ctx: *mut ngx_event_busy_lock_ctx_t)
    -> ngx_int_t;
   pub fn ngx_event_busy_lock_cacheable(bl: *mut ngx_event_busy_lock_t,
                                        ctx: *mut ngx_event_busy_lock_ctx_t)
    -> ngx_int_t;
   pub fn ngx_event_busy_unlock(bl: *mut ngx_event_busy_lock_t,
                                ctx: *mut ngx_event_busy_lock_ctx_t) -> ();
   pub fn ngx_event_busy_lock_cancel(bl: *mut ngx_event_busy_lock_t,
                                     ctx: *mut ngx_event_busy_lock_ctx_t)
    -> ();
   pub fn ngx_event_connect_peer(pc: *mut ngx_peer_connection_t)
    -> ngx_int_t;
   pub fn ngx_event_get_peer(pc: *mut ngx_peer_connection_t,
                             data: *mut ::libc::c_void) -> ngx_int_t;
   pub fn ngx_event_pipe(p: *mut ngx_event_pipe_t, do_write: ngx_int_t)
    -> ngx_int_t;
   pub fn ngx_event_pipe_copy_input_filter(p: *mut ngx_event_pipe_t,
                                           buf: *mut ngx_buf_t) -> ngx_int_t;
   pub fn ngx_event_pipe_add_free_buf(p: *mut ngx_event_pipe_t,
                                      b: *mut ngx_buf_t) -> ngx_int_t;
   pub fn ngx_http_upstream_cookie_variable(r: *mut ngx_http_request_t,
                                            v:
                                               *mut ngx_http_variable_value_t,
                                            data: uintptr_t) -> ngx_int_t;
   pub fn ngx_http_upstream_header_variable(r: *mut ngx_http_request_t,
                                            v:
                                               *mut ngx_http_variable_value_t,
                                            data: uintptr_t) -> ngx_int_t;
   pub fn ngx_http_upstream_create(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_upstream_init(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_upstream_add(cf: *mut ngx_conf_t, u: *mut ngx_url_t,
                                flags: ngx_uint_t)
    -> *mut ngx_http_upstream_srv_conf_t;
   pub fn ngx_http_upstream_bind_set_slot(cf: *mut ngx_conf_t,
                                          cmd: *mut ngx_command_t,
                                          conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_upstream_param_set_slot(cf: *mut ngx_conf_t,
                                           cmd: *mut ngx_command_t,
                                           conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_upstream_hide_headers_hash(cf: *mut ngx_conf_t,
                                              conf:
                                                  *mut ngx_http_upstream_conf_t,
                                              prev:
                                                  *mut ngx_http_upstream_conf_t,
                                              default_hide_headers:
                                                  *mut ngx_str_t,
                                              hash: *mut ngx_hash_init_t)
    -> ngx_int_t;
   pub fn ngx_http_upstream_init_round_robin(cf: *mut ngx_conf_t,
                                             us:
                                                 *mut ngx_http_upstream_srv_conf_t)
    -> ngx_int_t;
   pub fn ngx_http_upstream_init_round_robin_peer(r: *mut ngx_http_request_t,
                                                  us:
                                                      *mut ngx_http_upstream_srv_conf_t)
    -> ngx_int_t;
   pub fn ngx_http_upstream_create_round_robin_peer(r:
                                                        *mut ngx_http_request_t,
                                                    ur:
                                                        *mut ngx_http_upstream_resolved_t)
    -> ngx_int_t;
   pub fn ngx_http_upstream_get_round_robin_peer(pc:
                                                     *mut ngx_peer_connection_t,
                                                 data: *mut ::libc::c_void)
    -> ngx_int_t;
   pub fn ngx_http_upstream_free_round_robin_peer(pc:
                                                      *mut ngx_peer_connection_t,
                                                  data: *mut ::libc::c_void,
                                                  state: ngx_uint_t) -> ();
   pub fn ngx_http_upstream_set_round_robin_peer_session(pc:
                                                             *mut ngx_peer_connection_t,
                                                         data:
                                                             *mut ::libc::c_void)
    -> ngx_int_t;
   pub fn ngx_http_upstream_save_round_robin_peer_session(pc:
                                                              *mut ngx_peer_connection_t,
                                                          data:
                                                              *mut ::libc::c_void)
    -> ();
   pub fn ngx_http_busy_lock(bl: *mut ngx_http_busy_lock_t,
                             bc: *mut ngx_http_busy_lock_ctx_t)
    -> ::libc::c_int;
   pub fn ngx_http_busy_lock_cacheable(bl: *mut ngx_http_busy_lock_t,
                                       bc: *mut ngx_http_busy_lock_ctx_t,
                                       lock: ::libc::c_int) -> ::libc::c_int;
   pub fn ngx_http_busy_unlock(bl: *mut ngx_http_busy_lock_t,
                              bc: *mut ngx_http_busy_lock_ctx_t) -> ();
   pub fn ngx_http_set_busy_lock_slot(cf: *mut ngx_conf_t,
                                      cmd: *mut ngx_command_t,
                                      conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_core_run_phases(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_core_generic_phase(r: *mut ngx_http_request_t,
                                      ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_rewrite_phase(r: *mut ngx_http_request_t,
                                      ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_find_config_phase(r: *mut ngx_http_request_t,
                                          ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_post_rewrite_phase(r: *mut ngx_http_request_t,
                                           ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_access_phase(r: *mut ngx_http_request_t,
                                     ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_post_access_phase(r: *mut ngx_http_request_t,
                                          ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_try_files_phase(r: *mut ngx_http_request_t,
                                        ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_core_content_phase(r: *mut ngx_http_request_t,
                                      ph: *mut ngx_http_phase_handler_t)
    -> ngx_int_t;
   pub fn ngx_http_test_content_type(r: *mut ngx_http_request_t,
                                     types_hash: *mut ngx_hash_t)
    -> *mut ::libc::c_void;
   pub fn ngx_http_set_content_type(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_set_exten(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_set_etag(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_weak_etag(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_send_response(r: *mut ngx_http_request_t,
                                 status: ngx_uint_t, ct: *mut ngx_str_t,
                                 cv: *mut ngx_http_complex_value_t)
    -> ngx_int_t;
   pub fn ngx_http_map_uri_to_path(r: *mut ngx_http_request_t,
                                   name: *mut ngx_str_t,
                                   root_length: *mut size_t,
                                   reserved: size_t) -> *mut u_char;
   pub fn ngx_http_auth_basic_user(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_gzip_ok(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_subrequest(r: *mut ngx_http_request_t,
                              uri: *mut ngx_str_t, args: *mut ngx_str_t,
                              sr: *mut *mut ngx_http_request_t,
                              psr: *mut ngx_http_post_subrequest_t,
                              flags: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_http_internal_redirect(r: *mut ngx_http_request_t,
                                     uri: *mut ngx_str_t,
                                     args: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_http_named_location(r: *mut ngx_http_request_t,
                                  name: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_http_cleanup_add(r: *mut ngx_http_request_t, size: size_t)
    -> *mut ngx_http_cleanup_t;
   pub fn ngx_http_output_filter(r: *mut ngx_http_request_t,
                                 chain: *mut ngx_chain_t) -> ngx_int_t;
   pub fn ngx_http_write_filter(r: *mut ngx_http_request_t,
                                chain: *mut ngx_chain_t) -> ngx_int_t;
   pub fn ngx_http_set_disable_symlinks(r: *mut ngx_http_request_t,
                                        clcf: *mut ngx_http_core_loc_conf_t,
                                        path: *mut ngx_str_t,
                                        of: *mut ngx_open_file_info_t)
    -> ngx_int_t;
   pub fn ngx_http_get_forwarded_addr(r: *mut ngx_http_request_t,
                                      addr: *mut ngx_addr_t,
                                      headers: *mut ngx_array_t,
                                      value: *mut ngx_str_t,
                                      proxies: *mut ngx_array_t,
                                      recursive: ::libc::c_int) -> ngx_int_t;
   pub fn ngx_http_file_cache_new(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_file_cache_create(r: *mut ngx_http_request_t)
    -> ngx_int_t;
   pub fn ngx_http_file_cache_create_key(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_file_cache_open(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_file_cache_set_header(r: *mut ngx_http_request_t,
                                         buf: *mut u_char) -> ngx_int_t;
   pub fn ngx_http_file_cache_update(r: *mut ngx_http_request_t,
                                     tf: *mut ngx_temp_file_t) -> ();
   pub fn ngx_http_file_cache_update_header(r: *mut ngx_http_request_t)
    -> ();
   pub fn ngx_http_cache_send(arg1: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_file_cache_free(c: *mut ngx_http_cache_t,
                                   tf: *mut ngx_temp_file_t) -> ();
   pub fn ngx_http_file_cache_valid(cache_valid: *mut ngx_array_t,
                                    status: ngx_uint_t) -> time_t;
   pub fn ngx_http_file_cache_set_slot(cf: *mut ngx_conf_t,
                                       cmd: *mut ngx_command_t,
                                       conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_file_cache_valid_set_slot(cf: *mut ngx_conf_t,
                                             cmd: *mut ngx_command_t,
                                             conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_add_location(cf: *mut ngx_conf_t,
                                locations: *mut *mut ngx_queue_t,
                                clcf: *mut ngx_http_core_loc_conf_t)
    -> ngx_int_t;
   pub fn ngx_http_add_listen(cf: *mut ngx_conf_t,
                              cscf: *mut ngx_http_core_srv_conf_t,
                              lsopt: *mut ngx_http_listen_opt_t)
    -> ngx_int_t;
   pub fn ngx_http_init_connection(c: *mut ngx_connection_t) -> ();
   pub fn ngx_http_close_connection(c: *mut ngx_connection_t) -> ();
   pub fn ngx_http_ssl_servername(ssl_conn: *mut SSL, ad: *mut ::libc::c_int,
                                  arg: *mut ::libc::c_void) -> ::libc::c_int;
   pub fn ngx_http_parse_request_line(r: *mut ngx_http_request_t,
                                      b: *mut ngx_buf_t) -> ngx_int_t;
   pub fn ngx_http_parse_uri(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_parse_complex_uri(r: *mut ngx_http_request_t,
                                     merge_slashes: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_http_parse_status_line(r: *mut ngx_http_request_t,
                                     b: *mut ngx_buf_t,
                                     status: *mut ngx_http_status_t)
    -> ngx_int_t;
   pub fn ngx_http_parse_unsafe_uri(r: *mut ngx_http_request_t,
                                    uri: *mut ngx_str_t,
                                    args: *mut ngx_str_t,
                                    flags: *mut ngx_uint_t) -> ngx_int_t;
   pub fn ngx_http_parse_header_line(r: *mut ngx_http_request_t,
                                     b: *mut ngx_buf_t,
                                     allow_underscores: ngx_uint_t)
    -> ngx_int_t;
   pub fn ngx_http_parse_multi_header_lines(headers: *mut ngx_array_t,
                                            name: *mut ngx_str_t,
                                            value: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_http_parse_set_cookie_lines(headers: *mut ngx_array_t,
                                          name: *mut ngx_str_t,
                                          value: *mut ngx_str_t)
    -> ngx_int_t;
   pub fn ngx_http_arg(r: *mut ngx_http_request_t, name: *mut u_char,
                       len: size_t, value: *mut ngx_str_t) -> ngx_int_t;
   pub fn ngx_http_split_args(r: *mut ngx_http_request_t,
                              uri: *mut ngx_str_t, args: *mut ngx_str_t)
    -> ();
   pub fn ngx_http_parse_chunked(r: *mut ngx_http_request_t,
                                 b: *mut ngx_buf_t,
                                 ctx: *mut ngx_http_chunked_t) -> ngx_int_t;
   pub fn ngx_http_create_request(c: *mut ngx_connection_t)
    -> *mut ngx_http_request_t;
   pub fn ngx_http_process_request_uri(r: *mut ngx_http_request_t)
    -> ngx_int_t;
   pub fn ngx_http_process_request_header(r: *mut ngx_http_request_t)
    -> ngx_int_t;
   pub fn ngx_http_process_request(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_update_location_config(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_handler(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_run_posted_requests(c: *mut ngx_connection_t) -> ();
   pub fn ngx_http_post_request(r: *mut ngx_http_request_t,
                                pr: *mut ngx_http_posted_request_t)
    -> ngx_int_t;
   pub fn ngx_http_finalize_request(r: *mut ngx_http_request_t,
                                    rc: ngx_int_t) -> ();
   pub fn ngx_http_free_request(r: *mut ngx_http_request_t, rc: ngx_int_t)
    -> ();
   pub fn ngx_http_empty_handler(wev: *mut ngx_event_t) -> ();
   pub fn ngx_http_request_empty_handler(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_send_special(r: *mut ngx_http_request_t,
                                flags: ngx_uint_t) -> ngx_int_t;
   pub fn ngx_http_read_client_request_body(r: *mut ngx_http_request_t,
                                            post_handler:
                                               ngx_http_client_body_handler_pt)
    -> ngx_int_t;
   pub fn ngx_http_send_header(r: *mut ngx_http_request_t) -> ngx_int_t;
   pub fn ngx_http_special_response_handler(r: *mut ngx_http_request_t,
                                            error: ngx_int_t) -> ngx_int_t;
   pub fn ngx_http_filter_finalize_request(r: *mut ngx_http_request_t,
                                           m: *mut ngx_module_t,
                                           error: ngx_int_t) -> ngx_int_t;
   pub fn ngx_http_clean_header(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_parse_time(value: *mut u_char, len: size_t) -> time_t;
   pub fn ngx_http_get_time(buf: *mut ::libc::c_char, t: time_t) -> size_t;
   pub fn ngx_http_discard_request_body(r: *mut ngx_http_request_t)
    -> ngx_int_t;
   pub fn ngx_http_discarded_request_body_handler(r: *mut ngx_http_request_t)
    -> ();
   pub fn ngx_http_block_reading(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_test_reading(r: *mut ngx_http_request_t) -> ();
   pub fn ngx_http_types_slot(cf: *mut ngx_conf_t, cmd: *mut ngx_command_t,
                              conf: *mut ::libc::c_void)
    -> *mut ::libc::c_char;
   pub fn ngx_http_merge_types(cf: *mut ngx_conf_t,
                              keys: *mut *mut ngx_array_t,
                              types_hash: *mut ngx_hash_t,
                              prev_keys: *mut *mut ngx_array_t,
                              prev_types_hash: *mut ngx_hash_t,
                              default_types: *mut ngx_str_t)
    -> *mut ::libc::c_char;
   pub fn ngx_http_set_default_types(cf: *mut ngx_conf_t,
                                     types: *mut *mut ngx_array_t,
                                     default_type: *mut ngx_str_t)
    -> ngx_int_t;
}
