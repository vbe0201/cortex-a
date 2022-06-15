//! Processor core registers

#![allow(unused_attributes)]
#![rustfmt::skip]

#[macro_use]
mod macros;

mod actlr_el1;
mod actlr_el2;
mod actlr_el3;
mod ccsidr_el1;
mod clidr_el1;
mod cntfrq_el0;
mod cnthctl_el2;
mod cntp_ctl_el0;
mod cntp_tval_el0;
mod cntpct_el0;
mod cntv_ctl_el0;
mod cntv_cval_el0;
mod cntv_tval_el0;
mod cntvct_el0;
mod cntvoff_el2;
mod csselr_el1;
mod currentel;
mod daif;
mod elr_el1;
mod elr_el2;
mod elr_el3;
mod esr_el1;
mod esr_el2;
mod far_el1;
mod far_el2;
mod fp;
mod hcr_el2;
mod id_aa64mmfr0_el1;
mod id_aa64isar0_el1;
mod id_aa64mmfr2_el1;
mod lr;
mod mair_el1;
mod mair_el2;
mod midr_el1;
mod mpidr_el1;
mod oslar_el1;
mod par_el1;
mod scr_el3;
mod sctlr_el1;
mod sctlr_el2;
mod sp;
mod sp_el0;
mod sp_el1;
mod spsel;
mod spsr_el1;
mod spsr_el2;
mod spsr_el3;
mod tcr_el1;
mod tcr_el2;
mod tpidr_el0;
mod tpidr_el1;
mod tpidrro_el0;
mod ttbr0_el1;
mod ttbr0_el2;
mod ttbr1_el1;
mod vbar_el1;
mod vbar_el2;

pub use actlr_el1::ACTLR_EL1;
pub use actlr_el2::ACTLR_EL2;
pub use actlr_el3::ACTLR_EL3;
pub use ccsidr_el1::CCSIDR_EL1;
pub use clidr_el1::CLIDR_EL1;
pub use cntfrq_el0::CNTFRQ_EL0;
pub use cnthctl_el2::CNTHCTL_EL2;
pub use cntp_ctl_el0::CNTP_CTL_EL0;
pub use cntp_tval_el0::CNTP_TVAL_EL0;
pub use cntpct_el0::CNTPCT_EL0;
pub use cntv_ctl_el0::CNTV_CTL_EL0;
pub use cntv_cval_el0::CNTV_CVAL_EL0;
pub use cntv_tval_el0::CNTV_TVAL_EL0;
pub use cntvct_el0::CNTVCT_EL0;
pub use cntvoff_el2::CNTVOFF_EL2;
pub use csselr_el1::CSSELR_EL1;
pub use currentel::CurrentEL;
pub use daif::DAIF;
pub use elr_el1::ELR_EL1;
pub use elr_el2::ELR_EL2;
pub use elr_el3::ELR_EL3;
pub use esr_el1::ESR_EL1;
pub use esr_el2::ESR_EL2;
pub use far_el1::FAR_EL1;
pub use far_el2::FAR_EL2;
pub use fp::FP;
pub use hcr_el2::HCR_EL2;
pub use id_aa64mmfr0_el1::ID_AA64MMFR0_EL1;
pub use id_aa64isar0_el1::ID_AA64ISAR0_EL1;
pub use id_aa64mmfr2_el1::ID_AA64MMFR2_EL1;
pub use lr::LR;
pub use mair_el1::MAIR_EL1;
pub use mair_el2::MAIR_EL2;
pub use midr_el1::MIDR_EL1;
pub use mpidr_el1::MPIDR_EL1;
pub use oslar_el1::OSLAR_EL1;
pub use par_el1::PAR_EL1;
pub use scr_el3::SCR_EL3;
pub use sctlr_el1::SCTLR_EL1;
pub use sctlr_el2::SCTLR_EL2;
pub use sp::SP;
pub use sp_el0::SP_EL0;
pub use sp_el1::SP_EL1;
pub use spsel::SPSel;
pub use spsr_el1::SPSR_EL1;
pub use spsr_el2::SPSR_EL2;
pub use spsr_el3::SPSR_EL3;
pub use tcr_el1::TCR_EL1;
pub use tcr_el2::TCR_EL2;
pub use tpidr_el0::TPIDR_EL0;
pub use tpidr_el1::TPIDR_EL1;
pub use tpidrro_el0::TPIDRRO_EL0;
pub use ttbr0_el1::TTBR0_EL1;
pub use ttbr0_el2::TTBR0_EL2;
pub use ttbr1_el1::TTBR1_EL1;
pub use vbar_el1::VBAR_EL1;
pub use vbar_el2::VBAR_EL2;
