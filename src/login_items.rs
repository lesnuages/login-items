use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
use core_foundation::base::{kCFAllocatorDefault, CFRelease, CFTypeRef};
use core_foundation::string::{
    kCFStringEncodingUTF8, CFStringCreateWithCString, CFStringGetCStringPtr, CFStringRef,
};
use core_foundation::url::{CFURLCopyPath, CFURLCreateFromFileSystemRepresentation, CFURLRef};
use core_services::{
    kLSSharedFileListGlobalLoginItems, kLSSharedFileListItemLast,
    kLSSharedFileListSessionLoginItems, FSRef, IconRef, LSSharedFileListCopySnapshot,
    LSSharedFileListCreate, LSSharedFileListInsertItemURL, LSSharedFileListItemCopyDisplayName,
    LSSharedFileListItemRef, LSSharedFileListItemRemove, LSSharedFileListItemResolve,
};
use std::ffi::CStr;
use std::mem::transmute;
use std::ptr;
use urlencoding::decode;

pub fn list_login_items(global: bool) -> String {
    let mut output: String = String::from("");
    let list_options = ptr::null();
    let mut seed_value: u32 = 0;
    let mut out_url: CFURLRef = ptr::null_mut();
    let out_ref = ptr::null_mut() as *mut FSRef;

    unsafe {
        let mut list_type: CFStringRef = kLSSharedFileListSessionLoginItems;
        if global {
            list_type = kLSSharedFileListGlobalLoginItems;
        }
        let login_items = LSSharedFileListCreate(kCFAllocatorDefault, list_type, list_options);
        if (login_items as CFTypeRef).is_null() {
            return output;
        }
        let login_items_array = LSSharedFileListCopySnapshot(login_items, &mut seed_value);
        let login_items_count = CFArrayGetCount(login_items_array);

        for i in 0..login_items_count {
            let item_ptr = CFArrayGetValueAtIndex(login_items_array, i);
            let item: LSSharedFileListItemRef = transmute(item_ptr);
            if (item as CFTypeRef).is_null() {
                CFRelease(login_items as CFTypeRef);
                return output;
            }

            if LSSharedFileListItemResolve(item, 0, &mut out_url, out_ref) == 0 {
                let cs_item_name = LSSharedFileListItemCopyDisplayName(item);
                let cs_item_name_ptr = CFStringGetCStringPtr(cs_item_name, kCFStringEncodingUTF8);
                let mut cs_item_name_value = "";
                if !cs_item_name_ptr.is_null() {
                    cs_item_name_value = CStr::from_ptr(cs_item_name_ptr).to_str().unwrap();
                }
                let out_path_ptr = CFURLCopyPath(out_url);
                let encoded_path_ptr = CFStringGetCStringPtr(out_path_ptr, kCFStringEncodingUTF8);
                let mut encoded_path = "";
                if !encoded_path_ptr.is_null() {
                    encoded_path = CStr::from_ptr(encoded_path_ptr).to_str().unwrap();
                }
                let decoded_path = decode(encoded_path).unwrap();
                output.push_str(format!("{}: {}\n", cs_item_name_value, decoded_path).as_str());
            }
            CFRelease(item as CFTypeRef);
        }
        CFRelease(login_items as CFTypeRef);
    }
    return output;
}

pub fn add_login_item(global: bool, name: &str, path: &str) -> bool {
    let list_options = ptr::null();
    let cs_item_name = unsafe { CStr::from_ptr(name.as_ptr() as *const i8) };
    let icon_ref: IconRef = ptr::null_mut();

    unsafe {
        let mut list_type: CFStringRef = kLSSharedFileListSessionLoginItems;
        if global {
            list_type = kLSSharedFileListGlobalLoginItems;
        }
        let login_items = LSSharedFileListCreate(kCFAllocatorDefault, list_type, list_options);
        if (login_items as CFTypeRef).is_null() {
            return false;
        }
        let item_name = CFStringCreateWithCString(
            kCFAllocatorDefault,
            cs_item_name.as_ptr(),
            kCFStringEncodingUTF8,
        );
        let path_url = CFURLCreateFromFileSystemRepresentation(
            kCFAllocatorDefault,
            path.as_ptr(),
            path.len() as isize,
            0,
        );
        let item = LSSharedFileListInsertItemURL(
            login_items,
            kLSSharedFileListItemLast,
            item_name,
            icon_ref,
            path_url,
            ptr::null(),
            ptr::null(),
        );
        if (item as CFTypeRef).is_null() {
            CFRelease(login_items as CFTypeRef);
            return false;
        }
        CFRelease(login_items as CFTypeRef);
    }
    return true;
}

pub fn rm_login_item(global: bool, name: &str, path: &str) -> bool {
    let list_options = ptr::null();
    let mut seed_value: u32 = 0;
    let mut out_url: CFURLRef = ptr::null_mut();
    let out_ref = ptr::null_mut() as *mut FSRef;

    unsafe {
        let mut list_type: CFStringRef = kLSSharedFileListSessionLoginItems;
        if global {
            list_type = kLSSharedFileListGlobalLoginItems;
        }
        let login_items = LSSharedFileListCreate(kCFAllocatorDefault, list_type, list_options);
        if (login_items as CFTypeRef).is_null() {
            return false;
        }
        let login_items_array = LSSharedFileListCopySnapshot(login_items, &mut seed_value);
        let login_items_count = CFArrayGetCount(login_items_array);

        for i in 0..login_items_count {
            let item_ptr = CFArrayGetValueAtIndex(login_items_array, i);
            let item: LSSharedFileListItemRef = transmute(item_ptr);
            if (item as CFTypeRef).is_null() {
                CFRelease(login_items as CFTypeRef);
                return false;
            }

            if LSSharedFileListItemResolve(item, 0, &mut out_url, out_ref) == 0 {
                let cs_item_name = LSSharedFileListItemCopyDisplayName(item);
                let cs_item_name_ptr = CFStringGetCStringPtr(cs_item_name, kCFStringEncodingUTF8);
                if !cs_item_name_ptr.is_null() {
                    let cs_item_name_value = CStr::from_ptr(cs_item_name_ptr).to_str().unwrap();
                    if cs_item_name_value == name {
                        if LSSharedFileListItemRemove(login_items, item) != 0 {
                            CFRelease(item as CFTypeRef);
                            CFRelease(login_items as CFTypeRef);
                            return false;
                        }
                    }
                }
                let out_path_ptr = CFURLCopyPath(out_url);
                let encoded_path_ptr = CFStringGetCStringPtr(out_path_ptr, kCFStringEncodingUTF8);
                if !encoded_path_ptr.is_null() {
                    let encoded_path = CStr::from_ptr(encoded_path_ptr).to_str().unwrap();
                    let decoded_path = decode(encoded_path).unwrap();
                    if decoded_path == path {
                        if LSSharedFileListItemRemove(login_items, item) != 0 {
                            CFRelease(item as CFTypeRef);
                            CFRelease(login_items as CFTypeRef);
                            return false;
                        }
                    }
                }
            }
            CFRelease(item as CFTypeRef);
        }
        CFRelease(login_items as CFTypeRef);
    }
    return true;
}
