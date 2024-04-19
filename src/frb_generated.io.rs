// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.32.

// Section: imports

use super::*;
use crate::api::simple::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIAutoDeleveraging(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIAutoDeleveraging>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIAutoDeleveraging(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIAutoDeleveraging>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIChangePubKey(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIChangePubKey>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIChangePubKey(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIChangePubKey>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContract(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContract>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContract(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContract>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContractMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContractMatching>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContractMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContractMatching>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContractPrice(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContractPrice>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIContractPrice(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIContractPrice>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIEthTxOption(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIEthTxOption>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIEthTxOption(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIEthTxOption>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIForcedExit(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIForcedExit>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIForcedExit(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIForcedExit>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIFunding(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIFunding>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIFunding(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIFunding>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIFundingInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIFundingInfo>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIFundingInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIFundingInfo>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFILiquidation(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFILiquidation>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFILiquidation(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFILiquidation>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIOrder(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIOrder>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIOrder(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIOrder>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIOrderMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIOrderMatching>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIOrderMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIOrderMatching>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIParameter(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIParameter>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIParameter(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIParameter>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFISigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFISigner>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFISigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFISigner>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFISpotPriceInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFISpotPriceInfo>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFISpotPriceInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFISpotPriceInfo>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFITransfer(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFITransfer>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFITransfer(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFITransfer>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIUpdateGlobalVar(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIUpdateGlobalVar>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIUpdateGlobalVar(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIUpdateGlobalVar>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIWallet(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIWallet>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIWallet(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIWallet>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIWithdraw(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIWithdraw>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIWithdraw(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIWithdraw>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIZkLinkSigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIZkLinkSigner>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFFIZkLinkSigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FFIZkLinkSigner>>::decrement_strong_count(ptr as _);
}
