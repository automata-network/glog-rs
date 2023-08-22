#[cfg(feature = "tstd")]
use std::prelude::v1::*;

use std::thread_local;

use core::cell::RefCell;

thread_local! {
    pub static REQ_ID: RefCell<ReqId> = RefCell::new(ReqId::default());
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReqId([u8; 8]);
static EMPTY: ReqId = ReqId([0_u8; 8]);

impl ReqId {
    pub fn gen() -> Self {
        let mut req_id = Self::default();
        req_id.init();
        req_id
    }

    pub fn new() -> ReqId {
        REQ_ID.with(|f| {
            *f.borrow_mut() = ReqId::gen();
            f.borrow().clone()
        })
    }

    pub fn get() -> ReqId {
        REQ_ID.with(|f| f.borrow().clone())
    }

    pub fn save(&self) -> ReqId {
        REQ_ID.with(|f| {
            let old = f.borrow().clone();
            *f.borrow_mut() = self.clone();
            old
        })
    }

    pub fn get_or_new() -> ReqId {
        let req_id = Self::get();
        if req_id.is_empty() {
            Self::new()
        } else {
            req_id
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == EMPTY.0
    }

    #[cfg(feature = "tstd")]
    fn init(&mut self) {
        unsafe {
            use sgxlib::sgx_types::sgx_read_rand;
            use sgxlib::sgx_types::sgx_status_t;
            let s = sgx_read_rand(self.0.as_mut_ptr(), self.0.len());
            if s != sgx_status_t::SGX_SUCCESS {
                panic!("sgx unsupported: {}", s);
            }
        }
    }

    #[cfg(not(feature = "tstd"))]
    fn init(&mut self) {
        self.0 = rand::random();
    }
}

impl core::fmt::Debug for ReqId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl core::fmt::Display for ReqId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_req_id_std() {
        let id1 = ReqId::new();
        assert_eq!(id1.is_empty(), true);
        let handle = std::thread::spawn(|| {
            assert_eq!(ReqId::get().is_empty(), true);
        });
        let _ = handle.join();
        assert_eq!(format!("{:?}", ReqId::get()), format!("{:?}", id1));
    }
}
