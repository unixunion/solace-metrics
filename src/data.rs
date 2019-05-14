use solace_semp_client_monitor::models::{MsgVpnResponse, MsgVpn};
use serde::{Serialize, Serializer};


pub trait Data<T>: Serialize {
    fn data(&self) -> Option<&T>;
}


impl <'a, T> Data<T> for &'a T
where
T: Data<T>
{
    fn data(&self) -> Option<&T> {
        self.data()
    }
}


//
//impl Data<MsgVpn> for MsgVpnResponse where MsgVpnResponse: Serialize {
//    fn data(&self) -> Option<&MsgVpn> {
//        self.data()
//    }
//}


