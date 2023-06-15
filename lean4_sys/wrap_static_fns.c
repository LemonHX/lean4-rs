#include "lean.h"

// Static wrappers

bool lean_is_big_object_tag_rs_extern(uint8_t tag) { return lean_is_big_object_tag(tag); }
bool lean_is_scalar_rs_extern(lean_object *o) { return lean_is_scalar(o); }
lean_object * lean_box_rs_extern(size_t n) { return lean_box(n); }
size_t lean_unbox_rs_extern(lean_object *o) { return lean_unbox(o); }
size_t lean_align_rs_extern(size_t v, size_t a) { return lean_align(v, a); }
unsigned int lean_get_slot_idx_rs_extern(unsigned int sz) { return lean_get_slot_idx(sz); }
lean_object * lean_alloc_small_object_rs_extern(unsigned int sz) { return lean_alloc_small_object(sz); }
lean_object * lean_alloc_ctor_memory_rs_extern(unsigned int sz) { return lean_alloc_ctor_memory(sz); }
unsigned int lean_small_object_size_rs_extern(lean_object *o) { return lean_small_object_size(o); }
void lean_free_small_object_rs_extern(lean_object *o) { return lean_free_small_object(o); }
uint8_t lean_ptr_tag_rs_extern(lean_object *o) { return lean_ptr_tag(o); }
unsigned int lean_ptr_other_rs_extern(lean_object *o) { return lean_ptr_other(o); }
bool lean_is_mt_rs_extern(lean_object *o) { return lean_is_mt(o); }
bool lean_is_st_rs_extern(lean_object *o) { return lean_is_st(o); }
bool lean_is_persistent_rs_extern(lean_object *o) { return lean_is_persistent(o); }
bool lean_has_rc_rs_extern(lean_object *o) { return lean_has_rc(o); }
int * lean_get_rc_mt_addr_rs_extern(lean_object *o) { return lean_get_rc_mt_addr(o); }
void lean_inc_ref_rs_extern(lean_object *o) { return lean_inc_ref(o); }
void lean_inc_ref_n_rs_extern(lean_object *o, size_t n) { return lean_inc_ref_n(o, n); }
void lean_dec_ref_rs_extern(lean_object *o) { return lean_dec_ref(o); }
void lean_inc_rs_extern(lean_object *o) { return lean_inc(o); }
void lean_inc_n_rs_extern(lean_object *o, size_t n) { return lean_inc_n(o, n); }
void lean_dec_rs_extern(lean_object *o) { return lean_dec(o); }
bool lean_is_ctor_rs_extern(lean_object *o) { return lean_is_ctor(o); }
bool lean_is_closure_rs_extern(lean_object *o) { return lean_is_closure(o); }
bool lean_is_array_rs_extern(lean_object *o) { return lean_is_array(o); }
bool lean_is_sarray_rs_extern(lean_object *o) { return lean_is_sarray(o); }
bool lean_is_string_rs_extern(lean_object *o) { return lean_is_string(o); }
bool lean_is_mpz_rs_extern(lean_object *o) { return lean_is_mpz(o); }
bool lean_is_thunk_rs_extern(lean_object *o) { return lean_is_thunk(o); }
bool lean_is_task_rs_extern(lean_object *o) { return lean_is_task(o); }
bool lean_is_external_rs_extern(lean_object *o) { return lean_is_external(o); }
bool lean_is_ref_rs_extern(lean_object *o) { return lean_is_ref(o); }
unsigned int lean_obj_tag_rs_extern(lean_object *o) { return lean_obj_tag(o); }
lean_ctor_object * lean_to_ctor_rs_extern(lean_object *o) { return lean_to_ctor(o); }
lean_closure_object * lean_to_closure_rs_extern(lean_object *o) { return lean_to_closure(o); }
lean_array_object * lean_to_array_rs_extern(lean_object *o) { return lean_to_array(o); }
lean_sarray_object * lean_to_sarray_rs_extern(lean_object *o) { return lean_to_sarray(o); }
lean_string_object * lean_to_string_rs_extern(lean_object *o) { return lean_to_string(o); }
lean_thunk_object * lean_to_thunk_rs_extern(lean_object *o) { return lean_to_thunk(o); }
lean_task_object * lean_to_task_rs_extern(lean_object *o) { return lean_to_task(o); }
lean_ref_object * lean_to_ref_rs_extern(lean_object *o) { return lean_to_ref(o); }
lean_external_object * lean_to_external_rs_extern(lean_object *o) { return lean_to_external(o); }
bool lean_is_exclusive_rs_extern(lean_object *o) { return lean_is_exclusive(o); }
bool lean_is_shared_rs_extern(lean_object *o) { return lean_is_shared(o); }
void lean_set_st_header_rs_extern(lean_object *o, unsigned int tag, unsigned int other) { return lean_set_st_header(o, tag, other); }
void lean_set_non_heap_header_rs_extern(lean_object *o, size_t sz, unsigned int tag, unsigned int other) { return lean_set_non_heap_header(o, sz, tag, other); }
void lean_set_non_heap_header_for_big_rs_extern(lean_object *o, unsigned int tag, unsigned int other) { return lean_set_non_heap_header_for_big(o, tag, other); }
unsigned int lean_ctor_num_objs_rs_extern(lean_object *o) { return lean_ctor_num_objs(o); }
lean_object ** lean_ctor_obj_cptr_rs_extern(lean_object *o) { return lean_ctor_obj_cptr(o); }
uint8_t * lean_ctor_scalar_cptr_rs_extern(lean_object *o) { return lean_ctor_scalar_cptr(o); }
lean_object * lean_alloc_ctor_rs_extern(unsigned int tag, unsigned int num_objs, unsigned int scalar_sz) { return lean_alloc_ctor(tag, num_objs, scalar_sz); }
b_lean_obj_res lean_ctor_get_rs_extern(b_lean_obj_arg o, unsigned int i) { return lean_ctor_get(o, i); }
void lean_ctor_set_rs_extern(b_lean_obj_arg o, unsigned int i, lean_obj_arg v) { return lean_ctor_set(o, i, v); }
void lean_ctor_set_tag_rs_extern(b_lean_obj_arg o, uint8_t new_tag) { return lean_ctor_set_tag(o, new_tag); }
void lean_ctor_release_rs_extern(b_lean_obj_arg o, unsigned int i) { return lean_ctor_release(o, i); }
size_t lean_ctor_get_usize_rs_extern(b_lean_obj_arg o, unsigned int i) { return lean_ctor_get_usize(o, i); }
uint8_t lean_ctor_get_uint8_rs_extern(b_lean_obj_arg o, unsigned int offset) { return lean_ctor_get_uint8(o, offset); }
uint16_t lean_ctor_get_uint16_rs_extern(b_lean_obj_arg o, unsigned int offset) { return lean_ctor_get_uint16(o, offset); }
uint32_t lean_ctor_get_uint32_rs_extern(b_lean_obj_arg o, unsigned int offset) { return lean_ctor_get_uint32(o, offset); }
uint64_t lean_ctor_get_uint64_rs_extern(b_lean_obj_arg o, unsigned int offset) { return lean_ctor_get_uint64(o, offset); }
double lean_ctor_get_float_rs_extern(b_lean_obj_arg o, unsigned int offset) { return lean_ctor_get_float(o, offset); }
void lean_ctor_set_usize_rs_extern(b_lean_obj_arg o, unsigned int i, size_t v) { return lean_ctor_set_usize(o, i, v); }
void lean_ctor_set_uint8_rs_extern(b_lean_obj_arg o, unsigned int offset, uint8_t v) { return lean_ctor_set_uint8(o, offset, v); }
void lean_ctor_set_uint16_rs_extern(b_lean_obj_arg o, unsigned int offset, uint16_t v) { return lean_ctor_set_uint16(o, offset, v); }
void lean_ctor_set_uint32_rs_extern(b_lean_obj_arg o, unsigned int offset, uint32_t v) { return lean_ctor_set_uint32(o, offset, v); }
void lean_ctor_set_uint64_rs_extern(b_lean_obj_arg o, unsigned int offset, uint64_t v) { return lean_ctor_set_uint64(o, offset, v); }
void lean_ctor_set_float_rs_extern(b_lean_obj_arg o, unsigned int offset, double v) { return lean_ctor_set_float(o, offset, v); }
void * lean_closure_fun_rs_extern(lean_object *o) { return lean_closure_fun(o); }
unsigned int lean_closure_arity_rs_extern(lean_object *o) { return lean_closure_arity(o); }
unsigned int lean_closure_num_fixed_rs_extern(lean_object *o) { return lean_closure_num_fixed(o); }
lean_object ** lean_closure_arg_cptr_rs_extern(lean_object *o) { return lean_closure_arg_cptr(o); }
lean_obj_res lean_alloc_closure_rs_extern(void *fun, unsigned int arity, unsigned int num_fixed) { return lean_alloc_closure(fun, arity, num_fixed); }
b_lean_obj_res lean_closure_get_rs_extern(b_lean_obj_arg o, unsigned int i) { return lean_closure_get(o, i); }
void lean_closure_set_rs_extern(u_lean_obj_arg o, unsigned int i, lean_obj_arg a) { return lean_closure_set(o, i, a); }
lean_obj_res lean_alloc_array_rs_extern(size_t size, size_t capacity) { return lean_alloc_array(size, capacity); }
size_t lean_array_size_rs_extern(b_lean_obj_arg o) { return lean_array_size(o); }
size_t lean_array_capacity_rs_extern(b_lean_obj_arg o) { return lean_array_capacity(o); }
size_t lean_array_byte_size_rs_extern(lean_object *o) { return lean_array_byte_size(o); }
lean_object ** lean_array_cptr_rs_extern(lean_object *o) { return lean_array_cptr(o); }
void lean_array_set_size_rs_extern(u_lean_obj_arg o, size_t sz) { return lean_array_set_size(o, sz); }
b_lean_obj_res lean_array_get_core_rs_extern(b_lean_obj_arg o, size_t i) { return lean_array_get_core(o, i); }
void lean_array_set_core_rs_extern(u_lean_obj_arg o, size_t i, lean_obj_arg v) { return lean_array_set_core(o, i, v); }
lean_object * lean_array_sz_rs_extern(lean_obj_arg a) { return lean_array_sz(a); }
lean_object * lean_array_get_size_rs_extern(b_lean_obj_arg a) { return lean_array_get_size(a); }
lean_object * lean_mk_empty_array_rs_extern(void) { return lean_mk_empty_array(); }
lean_object * lean_mk_empty_array_with_capacity_rs_extern(b_lean_obj_arg capacity) { return lean_mk_empty_array_with_capacity(capacity); }
lean_object * lean_array_uget_rs_extern(b_lean_obj_arg a, size_t i) { return lean_array_uget(a, i); }
lean_obj_res lean_array_fget_rs_extern(b_lean_obj_arg a, b_lean_obj_arg i) { return lean_array_fget(a, i); }
lean_object * lean_array_get_rs_extern(lean_obj_arg def_val, b_lean_obj_arg a, b_lean_obj_arg i) { return lean_array_get(def_val, a, i); }
lean_obj_res lean_copy_array_rs_extern(lean_obj_arg a) { return lean_copy_array(a); }
lean_obj_res lean_ensure_exclusive_array_rs_extern(lean_obj_arg a) { return lean_ensure_exclusive_array(a); }
lean_object * lean_array_uset_rs_extern(lean_obj_arg a, size_t i, lean_obj_arg v) { return lean_array_uset(a, i, v); }
lean_object * lean_array_fset_rs_extern(lean_obj_arg a, b_lean_obj_arg i, lean_obj_arg v) { return lean_array_fset(a, i, v); }
lean_object * lean_array_set_rs_extern(lean_obj_arg a, b_lean_obj_arg i, lean_obj_arg v) { return lean_array_set(a, i, v); }
lean_object * lean_array_pop_rs_extern(lean_obj_arg a) { return lean_array_pop(a); }
lean_object * lean_array_uswap_rs_extern(lean_obj_arg a, size_t i, size_t j) { return lean_array_uswap(a, i, j); }
lean_object * lean_array_fswap_rs_extern(lean_obj_arg a, b_lean_obj_arg i, b_lean_obj_arg j) { return lean_array_fswap(a, i, j); }
lean_object * lean_array_swap_rs_extern(lean_obj_arg a, b_lean_obj_arg i, b_lean_obj_arg j) { return lean_array_swap(a, i, j); }
lean_obj_res lean_alloc_sarray_rs_extern(unsigned int elem_size, size_t size, size_t capacity) { return lean_alloc_sarray(elem_size, size, capacity); }
unsigned int lean_sarray_elem_size_rs_extern(lean_object *o) { return lean_sarray_elem_size(o); }
size_t lean_sarray_capacity_rs_extern(lean_object *o) { return lean_sarray_capacity(o); }
size_t lean_sarray_byte_size_rs_extern(lean_object *o) { return lean_sarray_byte_size(o); }
size_t lean_sarray_size_rs_extern(b_lean_obj_arg o) { return lean_sarray_size(o); }
void lean_sarray_set_size_rs_extern(u_lean_obj_arg o, size_t sz) { return lean_sarray_set_size(o, sz); }
uint8_t * lean_sarray_cptr_rs_extern(lean_object *o) { return lean_sarray_cptr(o); }
lean_obj_res lean_mk_empty_byte_array_rs_extern(b_lean_obj_arg capacity) { return lean_mk_empty_byte_array(capacity); }
lean_obj_res lean_byte_array_size_rs_extern(b_lean_obj_arg a) { return lean_byte_array_size(a); }
uint8_t lean_byte_array_uget_rs_extern(b_lean_obj_arg a, size_t i) { return lean_byte_array_uget(a, i); }
uint8_t lean_byte_array_get_rs_extern(b_lean_obj_arg a, b_lean_obj_arg i) { return lean_byte_array_get(a, i); }
uint8_t lean_byte_array_fget_rs_extern(b_lean_obj_arg a, b_lean_obj_arg i) { return lean_byte_array_fget(a, i); }
lean_object * lean_byte_array_uset_rs_extern(lean_obj_arg a, size_t i, uint8_t v) { return lean_byte_array_uset(a, i, v); }
lean_obj_res lean_byte_array_set_rs_extern(lean_obj_arg a, b_lean_obj_arg i, uint8_t b) { return lean_byte_array_set(a, i, b); }
lean_obj_res lean_byte_array_fset_rs_extern(lean_obj_arg a, b_lean_obj_arg i, uint8_t b) { return lean_byte_array_fset(a, i, b); }
lean_obj_res lean_mk_empty_float_array_rs_extern(b_lean_obj_arg capacity) { return lean_mk_empty_float_array(capacity); }
lean_obj_res lean_float_array_size_rs_extern(b_lean_obj_arg a) { return lean_float_array_size(a); }
double * lean_float_array_cptr_rs_extern(b_lean_obj_arg a) { return lean_float_array_cptr(a); }
double lean_float_array_uget_rs_extern(b_lean_obj_arg a, size_t i) { return lean_float_array_uget(a, i); }
double lean_float_array_fget_rs_extern(b_lean_obj_arg a, b_lean_obj_arg i) { return lean_float_array_fget(a, i); }
double lean_float_array_get_rs_extern(b_lean_obj_arg a, b_lean_obj_arg i) { return lean_float_array_get(a, i); }
lean_obj_res lean_float_array_uset_rs_extern(lean_obj_arg a, size_t i, double d) { return lean_float_array_uset(a, i, d); }
lean_obj_res lean_float_array_fset_rs_extern(lean_obj_arg a, b_lean_obj_arg i, double d) { return lean_float_array_fset(a, i, d); }
lean_obj_res lean_float_array_set_rs_extern(lean_obj_arg a, b_lean_obj_arg i, double d) { return lean_float_array_set(a, i, d); }
lean_obj_res lean_alloc_string_rs_extern(size_t size, size_t capacity, size_t len) { return lean_alloc_string(size, capacity, len); }
size_t lean_string_capacity_rs_extern(lean_object *o) { return lean_string_capacity(o); }
size_t lean_string_byte_size_rs_extern(lean_object *o) { return lean_string_byte_size(o); }
uint32_t lean_char_default_value_rs_extern(void) { return lean_char_default_value(); }
const char * lean_string_cstr_rs_extern(b_lean_obj_arg o) { return lean_string_cstr(o); }
size_t lean_string_size_rs_extern(b_lean_obj_arg o) { return lean_string_size(o); }
size_t lean_string_len_rs_extern(b_lean_obj_arg o) { return lean_string_len(o); }
lean_obj_res lean_string_length_rs_extern(b_lean_obj_arg s) { return lean_string_length(s); }
uint32_t lean_string_utf8_get_fast_rs_extern(b_lean_obj_arg s, b_lean_obj_arg i) { return lean_string_utf8_get_fast(s, i); }
lean_obj_res lean_string_utf8_next_fast_rs_extern(b_lean_obj_arg s, b_lean_obj_arg i) { return lean_string_utf8_next_fast(s, i); }
uint8_t lean_string_utf8_at_end_rs_extern(b_lean_obj_arg s, b_lean_obj_arg i) { return lean_string_utf8_at_end(s, i); }
lean_obj_res lean_string_utf8_byte_size_rs_extern(b_lean_obj_arg s) { return lean_string_utf8_byte_size(s); }
bool lean_string_eq_rs_extern(b_lean_obj_arg s1, b_lean_obj_arg s2) { return lean_string_eq(s1, s2); }
bool lean_string_ne_rs_extern(b_lean_obj_arg s1, b_lean_obj_arg s2) { return lean_string_ne(s1, s2); }
uint8_t lean_string_dec_eq_rs_extern(b_lean_obj_arg s1, b_lean_obj_arg s2) { return lean_string_dec_eq(s1, s2); }
uint8_t lean_string_dec_lt_rs_extern(b_lean_obj_arg s1, b_lean_obj_arg s2) { return lean_string_dec_lt(s1, s2); }
lean_obj_res lean_mk_thunk_rs_extern(lean_obj_arg c) { return lean_mk_thunk(c); }
lean_obj_res lean_thunk_pure_rs_extern(lean_obj_arg v) { return lean_thunk_pure(v); }
b_lean_obj_res lean_thunk_get_rs_extern(b_lean_obj_arg t) { return lean_thunk_get(t); }
lean_obj_res lean_thunk_get_own_rs_extern(b_lean_obj_arg t) { return lean_thunk_get_own(t); }
lean_obj_res lean_task_spawn_rs_extern(lean_obj_arg c, lean_obj_arg prio) { return lean_task_spawn(c, prio); }
lean_obj_res lean_task_bind_rs_extern(lean_obj_arg x, lean_obj_arg f, lean_obj_arg prio) { return lean_task_bind(x, f, prio); }
lean_obj_res lean_task_map_rs_extern(lean_obj_arg f, lean_obj_arg t, lean_obj_arg prio) { return lean_task_map(f, t, prio); }
lean_obj_res lean_task_get_own_rs_extern(lean_obj_arg t) { return lean_task_get_own(t); }
lean_object * lean_alloc_external_rs_extern(lean_external_class *cls, void *data) { return lean_alloc_external(cls, data); }
lean_external_class * lean_get_external_class_rs_extern(lean_object *o) { return lean_get_external_class(o); }
void * lean_get_external_data_rs_extern(lean_object *o) { return lean_get_external_data(o); }
lean_obj_res lean_usize_to_nat_rs_extern(size_t n) { return lean_usize_to_nat(n); }
lean_obj_res lean_unsigned_to_nat_rs_extern(unsigned int n) { return lean_unsigned_to_nat(n); }
lean_obj_res lean_uint64_to_nat_rs_extern(uint64_t n) { return lean_uint64_to_nat(n); }
lean_obj_res lean_nat_succ_rs_extern(b_lean_obj_arg a) { return lean_nat_succ(a); }
lean_obj_res lean_nat_add_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_add(a1, a2); }
lean_obj_res lean_nat_sub_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_sub(a1, a2); }
lean_obj_res lean_nat_mul_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_mul(a1, a2); }
lean_obj_res lean_nat_div_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_div(a1, a2); }
lean_obj_res lean_nat_mod_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_mod(a1, a2); }
bool lean_nat_eq_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_eq(a1, a2); }
uint8_t lean_nat_dec_eq_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_dec_eq(a1, a2); }
bool lean_nat_ne_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_ne(a1, a2); }
bool lean_nat_le_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_le(a1, a2); }
uint8_t lean_nat_dec_le_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_dec_le(a1, a2); }
bool lean_nat_lt_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_lt(a1, a2); }
uint8_t lean_nat_dec_lt_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_dec_lt(a1, a2); }
lean_obj_res lean_nat_land_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_land(a1, a2); }
lean_obj_res lean_nat_lor_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_lor(a1, a2); }
lean_obj_res lean_nat_lxor_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_nat_lxor(a1, a2); }
lean_obj_res lean_int_to_int_rs_extern(int n) { return lean_int_to_int(n); }
lean_obj_res lean_int64_to_int_rs_extern(int64_t n) { return lean_int64_to_int(n); }
int64_t lean_scalar_to_int64_rs_extern(b_lean_obj_arg a) { return lean_scalar_to_int64(a); }
int lean_scalar_to_int_rs_extern(b_lean_obj_arg a) { return lean_scalar_to_int(a); }
lean_obj_res lean_nat_to_int_rs_extern(lean_obj_arg a) { return lean_nat_to_int(a); }
lean_obj_res lean_int_neg_rs_extern(b_lean_obj_arg a) { return lean_int_neg(a); }
lean_obj_res lean_int_neg_succ_of_nat_rs_extern(lean_obj_arg a) { return lean_int_neg_succ_of_nat(a); }
lean_obj_res lean_int_add_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_add(a1, a2); }
lean_obj_res lean_int_sub_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_sub(a1, a2); }
lean_obj_res lean_int_mul_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_mul(a1, a2); }
lean_obj_res lean_int_div_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_div(a1, a2); }
lean_obj_res lean_int_mod_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_mod(a1, a2); }
bool lean_int_eq_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_eq(a1, a2); }
bool lean_int_ne_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_ne(a1, a2); }
bool lean_int_le_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_le(a1, a2); }
bool lean_int_lt_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_lt(a1, a2); }
lean_obj_res lean_int_to_nat_rs_extern(lean_obj_arg a) { return lean_int_to_nat(a); }
lean_obj_res lean_nat_abs_rs_extern(b_lean_obj_arg i) { return lean_nat_abs(i); }
uint8_t lean_int_dec_eq_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_dec_eq(a1, a2); }
uint8_t lean_int_dec_le_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_dec_le(a1, a2); }
uint8_t lean_int_dec_lt_rs_extern(b_lean_obj_arg a1, b_lean_obj_arg a2) { return lean_int_dec_lt(a1, a2); }
uint8_t lean_int_dec_nonneg_rs_extern(b_lean_obj_arg a) { return lean_int_dec_nonneg(a); }
uint64_t lean_bool_to_uint64_rs_extern(uint8_t a) { return lean_bool_to_uint64(a); }
uint8_t lean_uint8_of_nat_rs_extern(b_lean_obj_arg a) { return lean_uint8_of_nat(a); }
uint8_t lean_uint8_of_nat_mk_rs_extern(lean_obj_arg a) { return lean_uint8_of_nat_mk(a); }
lean_obj_res lean_uint8_to_nat_rs_extern(uint8_t a) { return lean_uint8_to_nat(a); }
uint8_t lean_uint8_add_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_add(a1, a2); }
uint8_t lean_uint8_sub_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_sub(a1, a2); }
uint8_t lean_uint8_mul_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_mul(a1, a2); }
uint8_t lean_uint8_div_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_div(a1, a2); }
uint8_t lean_uint8_mod_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_mod(a1, a2); }
uint8_t lean_uint8_land_rs_extern(uint8_t a, uint8_t b) { return lean_uint8_land(a, b); }
uint8_t lean_uint8_lor_rs_extern(uint8_t a, uint8_t b) { return lean_uint8_lor(a, b); }
uint8_t lean_uint8_xor_rs_extern(uint8_t a, uint8_t b) { return lean_uint8_xor(a, b); }
uint8_t lean_uint8_shift_left_rs_extern(uint8_t a, uint8_t b) { return lean_uint8_shift_left(a, b); }
uint8_t lean_uint8_shift_right_rs_extern(uint8_t a, uint8_t b) { return lean_uint8_shift_right(a, b); }
uint8_t lean_uint8_complement_rs_extern(uint8_t a) { return lean_uint8_complement(a); }
uint8_t lean_uint8_modn_rs_extern(uint8_t a1, b_lean_obj_arg a2) { return lean_uint8_modn(a1, a2); }
uint8_t lean_uint8_log2_rs_extern(uint8_t a) { return lean_uint8_log2(a); }
uint8_t lean_uint8_dec_eq_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_dec_eq(a1, a2); }
uint8_t lean_uint8_dec_lt_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_dec_lt(a1, a2); }
uint8_t lean_uint8_dec_le_rs_extern(uint8_t a1, uint8_t a2) { return lean_uint8_dec_le(a1, a2); }
uint16_t lean_uint8_to_uint16_rs_extern(uint8_t a) { return lean_uint8_to_uint16(a); }
uint32_t lean_uint8_to_uint32_rs_extern(uint8_t a) { return lean_uint8_to_uint32(a); }
uint64_t lean_uint8_to_uint64_rs_extern(uint8_t a) { return lean_uint8_to_uint64(a); }
uint16_t lean_uint16_of_nat_rs_extern(b_lean_obj_arg a) { return lean_uint16_of_nat(a); }
uint16_t lean_uint16_of_nat_mk_rs_extern(lean_obj_arg a) { return lean_uint16_of_nat_mk(a); }
lean_obj_res lean_uint16_to_nat_rs_extern(uint16_t a) { return lean_uint16_to_nat(a); }
uint16_t lean_uint16_add_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_add(a1, a2); }
uint16_t lean_uint16_sub_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_sub(a1, a2); }
uint16_t lean_uint16_mul_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_mul(a1, a2); }
uint16_t lean_uint16_div_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_div(a1, a2); }
uint16_t lean_uint16_mod_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_mod(a1, a2); }
uint16_t lean_uint16_land_rs_extern(uint16_t a, uint16_t b) { return lean_uint16_land(a, b); }
uint16_t lean_uint16_lor_rs_extern(uint16_t a, uint16_t b) { return lean_uint16_lor(a, b); }
uint16_t lean_uint16_xor_rs_extern(uint16_t a, uint16_t b) { return lean_uint16_xor(a, b); }
uint16_t lean_uint16_shift_left_rs_extern(uint16_t a, uint16_t b) { return lean_uint16_shift_left(a, b); }
uint16_t lean_uint16_shift_right_rs_extern(uint16_t a, uint16_t b) { return lean_uint16_shift_right(a, b); }
uint16_t lean_uint16_complement_rs_extern(uint16_t a) { return lean_uint16_complement(a); }
uint16_t lean_uint16_modn_rs_extern(uint16_t a1, b_lean_obj_arg a2) { return lean_uint16_modn(a1, a2); }
uint16_t lean_uint16_log2_rs_extern(uint16_t a) { return lean_uint16_log2(a); }
uint8_t lean_uint16_dec_eq_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_dec_eq(a1, a2); }
uint8_t lean_uint16_dec_lt_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_dec_lt(a1, a2); }
uint8_t lean_uint16_dec_le_rs_extern(uint16_t a1, uint16_t a2) { return lean_uint16_dec_le(a1, a2); }
uint8_t lean_uint16_to_uint8_rs_extern(uint16_t a) { return lean_uint16_to_uint8(a); }
uint32_t lean_uint16_to_uint32_rs_extern(uint16_t a) { return lean_uint16_to_uint32(a); }
uint64_t lean_uint16_to_uint64_rs_extern(uint16_t a) { return lean_uint16_to_uint64(a); }
uint32_t lean_uint32_of_nat_rs_extern(b_lean_obj_arg a) { return lean_uint32_of_nat(a); }
uint32_t lean_uint32_of_nat_mk_rs_extern(lean_obj_arg a) { return lean_uint32_of_nat_mk(a); }
lean_obj_res lean_uint32_to_nat_rs_extern(uint32_t a) { return lean_uint32_to_nat(a); }
uint32_t lean_uint32_add_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_add(a1, a2); }
uint32_t lean_uint32_sub_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_sub(a1, a2); }
uint32_t lean_uint32_mul_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_mul(a1, a2); }
uint32_t lean_uint32_div_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_div(a1, a2); }
uint32_t lean_uint32_mod_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_mod(a1, a2); }
uint32_t lean_uint32_land_rs_extern(uint32_t a, uint32_t b) { return lean_uint32_land(a, b); }
uint32_t lean_uint32_lor_rs_extern(uint32_t a, uint32_t b) { return lean_uint32_lor(a, b); }
uint32_t lean_uint32_xor_rs_extern(uint32_t a, uint32_t b) { return lean_uint32_xor(a, b); }
uint32_t lean_uint32_shift_left_rs_extern(uint32_t a, uint32_t b) { return lean_uint32_shift_left(a, b); }
uint32_t lean_uint32_shift_right_rs_extern(uint32_t a, uint32_t b) { return lean_uint32_shift_right(a, b); }
uint32_t lean_uint32_complement_rs_extern(uint32_t a) { return lean_uint32_complement(a); }
uint32_t lean_uint32_modn_rs_extern(uint32_t a1, b_lean_obj_arg a2) { return lean_uint32_modn(a1, a2); }
uint32_t lean_uint32_log2_rs_extern(uint32_t a) { return lean_uint32_log2(a); }
uint8_t lean_uint32_dec_eq_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_dec_eq(a1, a2); }
uint8_t lean_uint32_dec_lt_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_dec_lt(a1, a2); }
uint8_t lean_uint32_dec_le_rs_extern(uint32_t a1, uint32_t a2) { return lean_uint32_dec_le(a1, a2); }
uint8_t lean_uint32_to_uint8_rs_extern(uint32_t a) { return lean_uint32_to_uint8(a); }
uint16_t lean_uint32_to_uint16_rs_extern(uint32_t a) { return lean_uint32_to_uint16(a); }
uint64_t lean_uint32_to_uint64_rs_extern(uint32_t a) { return lean_uint32_to_uint64(a); }
size_t lean_uint32_to_usize_rs_extern(uint32_t a) { return lean_uint32_to_usize(a); }
uint64_t lean_uint64_of_nat_rs_extern(b_lean_obj_arg a) { return lean_uint64_of_nat(a); }
uint64_t lean_uint64_of_nat_mk_rs_extern(lean_obj_arg a) { return lean_uint64_of_nat_mk(a); }
uint64_t lean_uint64_add_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_add(a1, a2); }
uint64_t lean_uint64_sub_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_sub(a1, a2); }
uint64_t lean_uint64_mul_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_mul(a1, a2); }
uint64_t lean_uint64_div_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_div(a1, a2); }
uint64_t lean_uint64_mod_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_mod(a1, a2); }
uint64_t lean_uint64_land_rs_extern(uint64_t a, uint64_t b) { return lean_uint64_land(a, b); }
uint64_t lean_uint64_lor_rs_extern(uint64_t a, uint64_t b) { return lean_uint64_lor(a, b); }
uint64_t lean_uint64_xor_rs_extern(uint64_t a, uint64_t b) { return lean_uint64_xor(a, b); }
uint64_t lean_uint64_shift_left_rs_extern(uint64_t a, uint64_t b) { return lean_uint64_shift_left(a, b); }
uint64_t lean_uint64_shift_right_rs_extern(uint64_t a, uint64_t b) { return lean_uint64_shift_right(a, b); }
uint64_t lean_uint64_complement_rs_extern(uint64_t a) { return lean_uint64_complement(a); }
uint64_t lean_uint64_modn_rs_extern(uint64_t a1, b_lean_obj_arg a2) { return lean_uint64_modn(a1, a2); }
uint64_t lean_uint64_log2_rs_extern(uint64_t a) { return lean_uint64_log2(a); }
uint8_t lean_uint64_dec_eq_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_dec_eq(a1, a2); }
uint8_t lean_uint64_dec_lt_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_dec_lt(a1, a2); }
uint8_t lean_uint64_dec_le_rs_extern(uint64_t a1, uint64_t a2) { return lean_uint64_dec_le(a1, a2); }
uint8_t lean_uint64_to_uint8_rs_extern(uint64_t a) { return lean_uint64_to_uint8(a); }
uint16_t lean_uint64_to_uint16_rs_extern(uint64_t a) { return lean_uint64_to_uint16(a); }
uint32_t lean_uint64_to_uint32_rs_extern(uint64_t a) { return lean_uint64_to_uint32(a); }
size_t lean_uint64_to_usize_rs_extern(uint64_t a) { return lean_uint64_to_usize(a); }
size_t lean_usize_of_nat_rs_extern(b_lean_obj_arg a) { return lean_usize_of_nat(a); }
size_t lean_usize_of_nat_mk_rs_extern(lean_obj_arg a) { return lean_usize_of_nat_mk(a); }
size_t lean_usize_add_rs_extern(size_t a1, size_t a2) { return lean_usize_add(a1, a2); }
size_t lean_usize_sub_rs_extern(size_t a1, size_t a2) { return lean_usize_sub(a1, a2); }
size_t lean_usize_mul_rs_extern(size_t a1, size_t a2) { return lean_usize_mul(a1, a2); }
size_t lean_usize_div_rs_extern(size_t a1, size_t a2) { return lean_usize_div(a1, a2); }
size_t lean_usize_mod_rs_extern(size_t a1, size_t a2) { return lean_usize_mod(a1, a2); }
size_t lean_usize_land_rs_extern(size_t a, size_t b) { return lean_usize_land(a, b); }
size_t lean_usize_lor_rs_extern(size_t a, size_t b) { return lean_usize_lor(a, b); }
size_t lean_usize_xor_rs_extern(size_t a, size_t b) { return lean_usize_xor(a, b); }
size_t lean_usize_shift_left_rs_extern(size_t a, size_t b) { return lean_usize_shift_left(a, b); }
size_t lean_usize_shift_right_rs_extern(size_t a, size_t b) { return lean_usize_shift_right(a, b); }
size_t lean_usize_complement_rs_extern(size_t a) { return lean_usize_complement(a); }
size_t lean_usize_modn_rs_extern(size_t a1, b_lean_obj_arg a2) { return lean_usize_modn(a1, a2); }
size_t lean_usize_log2_rs_extern(size_t a) { return lean_usize_log2(a); }
uint8_t lean_usize_dec_eq_rs_extern(size_t a1, size_t a2) { return lean_usize_dec_eq(a1, a2); }
uint8_t lean_usize_dec_lt_rs_extern(size_t a1, size_t a2) { return lean_usize_dec_lt(a1, a2); }
uint8_t lean_usize_dec_le_rs_extern(size_t a1, size_t a2) { return lean_usize_dec_le(a1, a2); }
uint32_t lean_usize_to_uint32_rs_extern(size_t a) { return lean_usize_to_uint32(a); }
uint64_t lean_usize_to_uint64_rs_extern(size_t a) { return lean_usize_to_uint64(a); }
lean_obj_res lean_box_uint32_rs_extern(uint32_t v) { return lean_box_uint32(v); }
unsigned int lean_unbox_uint32_rs_extern(b_lean_obj_arg o) { return lean_unbox_uint32(o); }
lean_obj_res lean_box_uint64_rs_extern(uint64_t v) { return lean_box_uint64(v); }
uint64_t lean_unbox_uint64_rs_extern(b_lean_obj_arg o) { return lean_unbox_uint64(o); }
lean_obj_res lean_box_usize_rs_extern(size_t v) { return lean_box_usize(v); }
size_t lean_unbox_usize_rs_extern(b_lean_obj_arg o) { return lean_unbox_usize(o); }
lean_obj_res lean_box_float_rs_extern(double v) { return lean_box_float(v); }
double lean_unbox_float_rs_extern(b_lean_obj_arg o) { return lean_unbox_float(o); }
lean_obj_res lean_io_mk_world_rs_extern(void) { return lean_io_mk_world(); }
bool lean_io_result_is_ok_rs_extern(b_lean_obj_arg r) { return lean_io_result_is_ok(r); }
bool lean_io_result_is_error_rs_extern(b_lean_obj_arg r) { return lean_io_result_is_error(r); }
b_lean_obj_res lean_io_result_get_value_rs_extern(b_lean_obj_arg r) { return lean_io_result_get_value(r); }
b_lean_obj_res lean_io_result_get_error_rs_extern(b_lean_obj_arg r) { return lean_io_result_get_error(r); }
lean_obj_res lean_io_result_mk_ok_rs_extern(lean_obj_arg a) { return lean_io_result_mk_ok(a); }
lean_obj_res lean_io_result_mk_error_rs_extern(lean_obj_arg e) { return lean_io_result_mk_error(e); }
size_t lean_ptr_addr_rs_extern(b_lean_obj_arg a) { return lean_ptr_addr(a); }
uint64_t lean_name_hash_ptr_rs_extern(b_lean_obj_arg n) { return lean_name_hash_ptr(n); }
uint64_t lean_name_hash_rs_extern(b_lean_obj_arg n) { return lean_name_hash(n); }
uint8_t lean_float_to_uint8_rs_extern(double a) { return lean_float_to_uint8(a); }
uint16_t lean_float_to_uint16_rs_extern(double a) { return lean_float_to_uint16(a); }
uint32_t lean_float_to_uint32_rs_extern(double a) { return lean_float_to_uint32(a); }
uint64_t lean_float_to_uint64_rs_extern(double a) { return lean_float_to_uint64(a); }
size_t lean_float_to_usize_rs_extern(double a) { return lean_float_to_usize(a); }
double lean_float_add_rs_extern(double a, double b) { return lean_float_add(a, b); }
double lean_float_sub_rs_extern(double a, double b) { return lean_float_sub(a, b); }
double lean_float_mul_rs_extern(double a, double b) { return lean_float_mul(a, b); }
double lean_float_div_rs_extern(double a, double b) { return lean_float_div(a, b); }
double lean_float_negate_rs_extern(double a) { return lean_float_negate(a); }
uint8_t lean_float_beq_rs_extern(double a, double b) { return lean_float_beq(a, b); }
uint8_t lean_float_decLe_rs_extern(double a, double b) { return lean_float_decLe(a, b); }
uint8_t lean_float_decLt_rs_extern(double a, double b) { return lean_float_decLt(a, b); }
double lean_uint64_to_float_rs_extern(uint64_t a) { return lean_uint64_to_float(a); }
size_t lean_data_hashmap_mk_idx_rs_extern(lean_object *sz, uint64_t hash) { return lean_data_hashmap_mk_idx(sz, hash); }
