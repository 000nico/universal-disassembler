use std::fs;

use md5;
use sha1::{Sha1, Digest};
use sha2::Sha256;

pub fn getMD5(aob: &[u8]) -> md5::Digest {
    md5::compute(aob)
}

pub fn get_sha1(data: &[u8]) -> sha1::digest::Output<Sha1> {
    Sha1::digest(data)
}

pub fn get_sha256(data: &[u8]) -> sha2::digest::Output<Sha256> {
    Sha256::digest(data)
}