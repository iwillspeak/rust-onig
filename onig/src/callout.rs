//! Callout Support
//!
//! Callouts are registered on a regex and provide notifications as a
//! regex is matched.

use std::ffi::{CStr, CString, NulError};

use onig_sys::OnigCalloutIn;

/// The Callout Arguments Structure
///
/// This opaque type models access to the underlying callout arguments.
#[derive(Debug)]
pub struct CalloutArgs {
    raw: *mut onig_sys::OnigCalloutArgs,
}

/// The Callout Arguments
///
/// This struct wraps the information passed in to each callback.
impl CalloutArgs {
    pub(crate) fn from_raw(raw: *mut onig_sys::OnigCalloutArgs) -> Self {
        CalloutArgs { raw }
    }

    /// Returns the callout number of this callout. "callout number" is an
    /// idneitifier of callout in a regex pattern.
    pub fn callout_num(&self) -> i32 {
        unsafe { onig_sys::onig_get_callout_num_by_callout_args(self.raw) }
    }

    /// Returns the direction of this callout.
    pub fn callout_in(&self) -> CalloutIn {
        CalloutIn::from(unsafe { onig_sys::onig_get_callout_in_by_callout_args(self.raw) })
    }

    /// Returns the name identifier of this callout. Fi this callout is callout
    /// of contents, then returns None.
    pub fn name_id(&self) -> Option<i32> {
        let name = unsafe { onig_sys::onig_get_name_id_by_callout_args(self.raw) };
        if name == onig_sys::ONIG_NON_NAME_ID {
            None
        } else {
            Some(name)
        }
    }

    /// Returns the contents tring of this callout.
    pub fn contents(&self) -> Option<CString> {
        let start = unsafe { onig_sys::onig_get_contents_by_callout_args(self.raw) };
        let end = unsafe { onig_sys::onig_get_contents_end_by_callout_args(self.raw) };
        if start.is_null() || end.is_null() {
            None
        } else {
            cstring_from_start_end(start, end).ok()
        }
    }

    /// Returns the subject string.
    pub fn subject(&self) -> CString {
        let start = unsafe { onig_sys::onig_get_string_by_callout_args(self.raw) };
        let end = unsafe { onig_sys::onig_get_string_end_by_callout_args(self.raw) };

        cstring_from_start_end(start, end).unwrap()
    }

    /// Returns the current counter value for retry-limit-in-match.
    pub fn retry_counter(&self) -> u64 {
        unsafe { onig_sys::onig_get_retry_counter_by_callout_args(self.raw) as u64 }
    }

    /// Returns current used match-stack size.
    ///
    /// The returned tuple `(used_num, used_bytes)` is made up of:
    ///
    ///   * `used_num` - number of match-srtack elements
    ///   * `used_bytes` - used byte size of match stack
    pub fn used_stack_size(&self) -> Option<(i32, i32)> {
        let mut used_num = 0;
        let mut used_bytes = 0;
        let r = unsafe {
            onig_sys::onig_get_used_stack_size_in_callout(
                self.raw,
                (&mut used_num) as *mut _,
                (&mut used_bytes) as *mut _,
            )
        };

        if r == 0 {
            Some((used_num, used_bytes))
        } else {
            None
        }
    }

    /// Returns current capture range position. Position is byte length offset
    /// from subject string. For uncaptured `mem_num` `None` is returned.
    ///
    ///  * `mem_num` - the capture group number to query for.
    pub fn capture_range(&self, mem_num: i32) -> Option<(i32, i32)> {
        let mut begin = 0;
        let mut end = 0;
        let r = unsafe {
            onig_sys::onig_get_capture_range_in_callout(
                self.raw,
                mem_num,
                &mut begin as *mut _,
                &mut end as *mut _,
            )
        };
        if r == 0 && begin != onig_sys::ONIG_REGION_NOTPOS && end != onig_sys::ONIG_REGION_NOTPOS {
            Some((begin, end))
        } else {
            None
        }
    }
}

/// Callout Type
#[derive(Debug)]
pub enum CalloutIn {
    /// The current callout is a progress callout.
    Progress,
    /// The current callout is a retraction callout.
    Retraction,
}

impl From<OnigCalloutIn> for CalloutIn {
    fn from(i: OnigCalloutIn) -> Self {
        match i {
            onig_sys::OnigCalloutIn_ONIG_CALLOUT_IN_PROGRESS => CalloutIn::Progress,
            onig_sys::OnigCalloutIn_ONIG_CALLOUT_IN_RETRACTION => CalloutIn::Retraction,
            _ => panic!("Invalid value for CalloutIn"),
        }
    }
}

/// Callout Result
pub enum CalloutResult {
    /// The callout succeeded. Matching should continue.
    Success,
    /// The callout failed. Matching should fail.
    Fail,
    /// The callout encountered an error.
    Error(u32),
}

impl From<CalloutResult> for ::std::os::raw::c_int {
    fn from(result: CalloutResult) -> Self {
        match result {
            CalloutResult::Success => 0,
            CalloutResult::Fail => 1,
            CalloutResult::Error(code) => -(code as i32),
        }
    }
}

/// The Callout Trait
///
/// Callouts can be registered to receieve a notification when regex
/// matches are in progress.
pub trait Callout {
    /// On Match Callback
    ///
    /// Called when a regex match meets the criteria this callout was
    /// registered for.
    fn on_match_progress(&self, args: CalloutArgs) -> CalloutResult;

    /// On Retraction Callback
    ///
    /// Called when a regex match retraction meets the criteria this callout was
    /// registered for.
    fn on_retraction(&mut self, args: CalloutArgs) -> CalloutResult;
}

impl<T> Callout for T
where
    T: Fn(CalloutArgs) -> CalloutResult,
{
    fn on_match_progress(&self, args: CalloutArgs) -> CalloutResult {
        self(args)
    }

    fn on_retraction(&mut self, args: CalloutArgs) -> CalloutResult {
        self(args)
    }
}

/// Returns the callout name for the given name id. If an invalid Id is passed
/// then `None` is returned.
pub fn get_callout_name(name_id: i32) -> Option<CString> {
    let name = unsafe { onig_sys::onig_get_callout_name_by_name_id(name_id) };
    if name.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(name as *mut _).into() })
    }
}

fn cstring_from_start_end(start: *const u8, end: *const u8) -> Result<CString, NulError> {
    unsafe {
        CString::new(core::slice::from_raw_parts(
            start,
            end.offset_from(start) as usize,
        ))
    }
}
