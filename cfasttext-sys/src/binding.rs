/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_args_t {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_prediction_t {
    pub prob: f32,
    pub label: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_predictions_t {
    pub predictions: *mut fasttext_prediction_t,
    pub length: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_tokens_t {
    pub tokens: *mut *mut ::std::os::raw::c_char,
    pub length: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fasttext_words_t {
    pub words: *const i32,
    pub length: usize,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum model_name_t {
    MODEL_CBOW = 1,
    MODEL_SG = 2,
    MODEL_SUP = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum loss_name_t {
    LOSS_HS = 1,
    LOSS_NS = 2,
    LOSS_SOFTMAX = 3,
    LOSS_OVA = 4,
}

extern "C" {
    pub fn cft_str_free(s: *mut ::std::os::raw::c_char);
    pub fn cft_args_new() -> *mut fasttext_args_t;
    pub fn cft_args_parse(
        handle: *mut fasttext_args_t,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_args_free(handle: *mut fasttext_args_t);
    pub fn cft_args_get_input(handle: *mut fasttext_args_t) -> *const ::std::os::raw::c_char;
    pub fn cft_args_set_input(handle: *mut fasttext_args_t, input: *const ::std::os::raw::c_char);
    pub fn cft_args_get_output(handle: *mut fasttext_args_t) -> *const ::std::os::raw::c_char;
    pub fn cft_args_set_output(handle: *mut fasttext_args_t, output: *const ::std::os::raw::c_char);
    pub fn cft_args_get_lr(handle: *mut fasttext_args_t) -> ::std::os::raw::c_double;
    pub fn cft_args_set_lr(handle: *mut fasttext_args_t, lr: ::std::os::raw::c_double);
    pub fn cft_args_get_lr_update_rate(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_lr_update_rate(handle: *mut fasttext_args_t, lr: ::std::os::raw::c_int);
    pub fn cft_args_get_dim(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_dim(handle: *mut fasttext_args_t, dim: ::std::os::raw::c_int);
    pub fn cft_args_get_ws(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_ws(handle: *mut fasttext_args_t, ws: ::std::os::raw::c_int);
    pub fn cft_args_get_epoch(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_epoch(handle: *mut fasttext_args_t, epoch: ::std::os::raw::c_int);
    pub fn cft_args_get_thread(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_thread(handle: *mut fasttext_args_t, thread: ::std::os::raw::c_int);
    pub fn cft_args_get_model(handle: *mut fasttext_args_t) -> model_name_t;
    pub fn cft_args_set_model(handle: *mut fasttext_args_t, model: model_name_t);
    pub fn cft_args_get_loss(handle: *mut fasttext_args_t) -> loss_name_t;
    pub fn cft_args_set_loss(handle: *mut fasttext_args_t, loss: loss_name_t);
    pub fn cft_args_get_min_count(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_min_count(handle: *mut fasttext_args_t, min_count: ::std::os::raw::c_int);
    pub fn cft_args_get_min_count_label(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_min_count_label(
        handle: *mut fasttext_args_t,
        min_count: ::std::os::raw::c_int,
    );
    pub fn cft_args_get_neg(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_neg(handle: *mut fasttext_args_t, neg: ::std::os::raw::c_int);
    pub fn cft_args_get_word_ngrams(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_word_ngrams(handle: *mut fasttext_args_t, ngrams: ::std::os::raw::c_int);
    pub fn cft_args_get_bucket(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_bucket(handle: *mut fasttext_args_t, bucket: ::std::os::raw::c_int);
    pub fn cft_args_get_minn(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_minn(handle: *mut fasttext_args_t, minn: ::std::os::raw::c_int);
    pub fn cft_args_get_maxn(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_maxn(handle: *mut fasttext_args_t, maxn: ::std::os::raw::c_int);
    pub fn cft_args_get_t(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_t(handle: *mut fasttext_args_t, t: ::std::os::raw::c_int);
    pub fn cft_args_get_verbose(handle: *mut fasttext_args_t) -> ::std::os::raw::c_int;
    pub fn cft_args_set_verbose(handle: *mut fasttext_args_t, verbose: ::std::os::raw::c_int);
    pub fn cft_args_get_label(handle: *mut fasttext_args_t) -> *const ::std::os::raw::c_char;
    pub fn cft_args_set_label(handle: *mut fasttext_args_t, label: *const ::std::os::raw::c_char);
    pub fn cft_args_get_save_output(handle: *mut fasttext_args_t) -> bool;
    pub fn cft_args_set_save_output(handle: *mut fasttext_args_t, save_output: bool);
    pub fn cft_args_get_qout(handle: *mut fasttext_args_t) -> bool;
    pub fn cft_args_set_qout(handle: *mut fasttext_args_t, qout: bool);
    pub fn cft_args_get_retrain(handle: *mut fasttext_args_t) -> bool;
    pub fn cft_args_set_retrain(handle: *mut fasttext_args_t, retrain: bool);
    pub fn cft_args_get_qnorm(handle: *mut fasttext_args_t) -> bool;
    pub fn cft_args_set_qnorm(handle: *mut fasttext_args_t, qnorm: bool);
    pub fn cft_args_get_cutoff(handle: *mut fasttext_args_t) -> usize;
    pub fn cft_args_set_cutoff(handle: *mut fasttext_args_t, cutoff: usize);
    pub fn cft_args_get_dsub(handle: *mut fasttext_args_t) -> usize;
    pub fn cft_args_set_dsub(handle: *mut fasttext_args_t, dsub: usize);
    pub fn cft_args_print_help(handle: *mut fasttext_args_t);
    pub fn cft_args_print_basic_help(handle: *mut fasttext_args_t);
    pub fn cft_args_print_dictionary_help(handle: *mut fasttext_args_t);
    pub fn cft_args_print_training_help(handle: *mut fasttext_args_t);
    pub fn cft_args_print_quantization_help(handle: *mut fasttext_args_t);

    pub fn cft_fasttext_new() -> *mut fasttext_t;
    pub fn cft_fasttext_free(handle: *mut fasttext_t);
    pub fn cft_fasttext_load_model(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_save_model(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_save_output(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_save_vectors(
        handle: *mut fasttext_t,
        filename: *const ::std::os::raw::c_char,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_get_dimension(handle: *mut fasttext_t) -> ::std::os::raw::c_int;
    pub fn cft_fasttext_get_word_id(
        handle: *mut fasttext_t,
        word: *const ::std::os::raw::c_char,
    ) -> i32;
    pub fn cft_fasttext_get_subword_id(
        handle: *mut fasttext_t,
        word: *const ::std::os::raw::c_char,
    ) -> i32;
    pub fn cft_fasttext_is_quant(handle: *mut fasttext_t) -> bool;
    pub fn cft_fasttext_train(
        handle: *mut fasttext_t,
        args: *mut fasttext_args_t,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_predict(
        handle: *mut fasttext_t,
        text: *const ::std::os::raw::c_char,
        k: i32,
        threshold: f32,
        errptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut fasttext_predictions_t;
    pub fn cft_fasttext_predict_on_words(
        handle: *mut fasttext_t,
        words: *const fasttext_words_t,
        k: i32,
        threshold: f32,
        errptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut fasttext_predictions_t;
    pub fn cft_fasttext_predictions_free(predictions: *mut fasttext_predictions_t);
    pub fn cft_fasttext_quantize(
        handle: *mut fasttext_t,
        args: *mut fasttext_args_t,
        errptr: *mut *mut ::std::os::raw::c_char,
    );
    pub fn cft_fasttext_get_word_vector(
        handle: *mut fasttext_t,
        word: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_float,
    );
    pub fn cft_fasttext_get_sentence_vector(
        handle: *mut fasttext_t,
        sentence: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_float,
    );
    pub fn cft_fasttext_abort(handle: *mut fasttext_t);
    pub fn cft_fasttext_tokenize(
        handle: *mut fasttext_t,
        text: *const ::std::os::raw::c_char,
    ) -> *mut fasttext_tokens_t;
    pub fn cft_fasttext_tokens_free(tokens: *mut fasttext_tokens_t);
}
