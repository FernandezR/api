// Pi-hole: A black hole for Internet advertisements
// (c) 2018 Pi-hole, LLC (https://pi-hole.net)
// Network-wide ad blocking via your own hardware.
//
// API
// FTL Shared Memory Query Structure
//
// This file is copyright under the latest version of the EUPL.
// Please see LICENSE file for your rights under this license.

use ftl::FtlQueryType;
use libc;

#[cfg(test)]
use ftl::memory_model::MAGIC_BYTE;

/// The query struct stored in shared memory
#[repr(C)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Copy, Clone)]
pub struct FtlQuery {
    magic: libc::c_uchar,
    pub timestamp: libc::time_t,
    pub time_index: libc::c_int,
    pub query_type: FtlQueryType,
    pub status: FtlQueryStatus,
    pub domain_id: libc::c_int,
    pub client_id: libc::c_int,
    pub upstream_id: libc::c_int,
    pub database_id: i64,
    pub id: libc::c_int,
    pub is_complete: bool,
    pub is_private: bool,
    /// Saved in units of 1/10 milliseconds (1 = 0.1ms, 2 = 0.2ms,
    /// 2500 = 250.0ms, etc.)
    pub response_time: libc::c_ulong,
    pub reply_type: FtlQueryReplyType,
    pub dnssec_type: FtlDnssecType,
    ad_bit: bool
}

impl FtlQuery {
    #[cfg(test)]
    pub fn new(
        id: isize,
        database_id: i64,
        timestamp: usize,
        time_index: usize,
        response_time: usize,
        domain_id: usize,
        client_id: usize,
        upstream_id: usize,
        query_type: FtlQueryType,
        status: FtlQueryStatus,
        reply_type: FtlQueryReplyType,
        dnssec_type: FtlDnssecType,
        is_complete: bool,
        is_private: bool
    ) -> FtlQuery {
        FtlQuery {
            magic: MAGIC_BYTE,
            id: id as libc::c_int,
            database_id,
            timestamp: timestamp as libc::time_t,
            time_index: time_index as libc::c_int,
            response_time: response_time as libc::c_ulong,
            domain_id: domain_id as libc::c_int,
            client_id: client_id as libc::c_int,
            upstream_id: upstream_id as libc::c_int,
            query_type,
            status,
            reply_type,
            dnssec_type,
            is_complete,
            is_private,
            ad_bit: false
        }
    }
}

/// The statuses an FTL query can have
#[repr(u8)]
#[cfg_attr(test, derive(Debug))]
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FtlQueryStatus {
    Unknown,
    Gravity,
    Forward,
    Cache,
    Wildcard,
    Blacklist,
    ExternalBlock
}

/// The reply types an FTL query can have
#[repr(u8)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum FtlQueryReplyType {
    Unknown,
    NODATA,
    NXDOMAIN,
    CNAME,
    IP,
    DOMAIN,
    RRNAME
}

/// The DNSSEC reply types an FTL query can have
#[repr(u8)]
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum FtlDnssecType {
    Unspecified,
    Secure,
    Insecure,
    Bogus,
    Abandoned,
    Unknown
}