use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(
    Default, Serialize, Deserialize, Eq, PartialEq, Debug, Copy, Clone, strum::VariantArray, strum::VariantNames, strum::Display
)]
pub enum SpdxLicense {
    #[serde(rename = "Glide")]
    #[strum(to_string = "Glide")]
    Glide = 0,
    #[serde(rename = "Abstyles")]
    #[strum(to_string = "Abstyles")]
    Abstyles,
    #[serde(rename = "AFL-1.1")]
    #[strum(to_string = "AFL-1.1")]
    AFL_1_1,
    #[serde(rename = "AFL-1.2")]
    #[strum(to_string = "AFL-1.2")]
    AFL_1_2,
    #[serde(rename = "AFL-2.0")]
    #[strum(to_string = "AFL-2.0")]
    AFL_2_0,
    #[serde(rename = "AFL-2.1")]
    #[strum(to_string = "AFL-2.1")]
    AFL_2_1,
    #[serde(rename = "AFL-3.0")]
    #[strum(to_string = "AFL-3.0")]
    AFL_3_0,
    #[serde(rename = "AMPAS")]
    #[strum(to_string = "AMPAS")]
    AMPAS,
    #[serde(rename = "AdaCore-doc")]
    #[strum(to_string = "AdaCore-doc")]
    AdaCore_doc,
    #[serde(rename = "APL-1.0")]
    #[strum(to_string = "APL-1.0")]
    APL_1_0,
    #[serde(rename = "Adobe-Glyph")]
    #[strum(to_string = "Adobe-Glyph")]
    Adobe_Glyph,
    #[serde(rename = "APAFML")]
    #[strum(to_string = "APAFML")]
    APAFML,
    #[serde(rename = "Adobe-2006")]
    #[strum(to_string = "Adobe-2006")]
    Adobe_2006,
    #[serde(rename = "Adobe-Utopia")]
    #[strum(to_string = "Adobe-Utopia")]
    Adobe_Utopia,
    #[serde(rename = "AGPL-1.0-only")]
    #[strum(to_string = "AGPL-1.0-only")]
    AGPL_1_0_only,
    #[serde(rename = "AGPL-1.0-or-later")]
    #[strum(to_string = "AGPL-1.0-or-later")]
    AGPL_1_0_or_later,
    #[serde(rename = "Afmparse")]
    #[strum(to_string = "Afmparse")]
    Afmparse,
    #[serde(rename = "Aladdin")]
    #[strum(to_string = "Aladdin")]
    Aladdin,
    #[serde(rename = "ADSL")]
    #[strum(to_string = "ADSL")]
    ADSL,
    #[serde(rename = "AMDPLPA")]
    #[strum(to_string = "AMDPLPA")]
    AMDPLPA,
    #[serde(rename = "ANTLR-PD")]
    #[strum(to_string = "ANTLR-PD")]
    ANTLR_PD,
    #[serde(rename = "ANTLR-PD-fallback")]
    #[strum(to_string = "ANTLR-PD-fallback")]
    ANTLR_PD_fallback,
    #[serde(rename = "Apache-1.0")]
    #[strum(to_string = "Apache-1.0")]
    Apache_1_0,
    #[serde(rename = "Apache-1.1")]
    #[strum(to_string = "Apache-1.1")]
    Apache_1_1,
    #[default]
    #[serde(rename = "Apache-2.0")]
    #[strum(to_string = "Apache-2.0")]
    Apache_2_0,
    #[serde(rename = "App-s2p")]
    #[strum(to_string = "App-s2p")]
    App_s2p,
    #[serde(rename = "AML")]
    #[strum(to_string = "AML")]
    AML,
    #[serde(rename = "APSL-1.0")]
    #[strum(to_string = "APSL-1.0")]
    APSL_1_0,
    #[serde(rename = "APSL-1.1")]
    #[strum(to_string = "APSL-1.1")]
    APSL_1_1,
    #[serde(rename = "APSL-1.2")]
    #[strum(to_string = "APSL-1.2")]
    APSL_1_2,
    #[serde(rename = "APSL-2.0")]
    #[strum(to_string = "APSL-2.0")]
    APSL_2_0,
    #[serde(rename = "Arphic-1999")]
    #[strum(to_string = "Arphic-1999")]
    Arphic_1999,
    #[serde(rename = "Artistic-1.0")]
    #[strum(to_string = "Artistic-1.0")]
    Artistic_1_0,
    #[serde(rename = "Artistic-1.0-Perl")]
    #[strum(to_string = "Artistic-1.0-Perl")]
    Artistic_1_0_Perl,
    #[serde(rename = "Artistic-1.0-cl8")]
    #[strum(to_string = "Artistic-1.0-cl8")]
    Artistic_1_0_cl8,
    #[serde(rename = "Artistic-2.0")]
    #[strum(to_string = "Artistic-2.0")]
    Artistic_2_0,
    #[serde(rename = "ASWF-Digital-Assets-1.1")]
    #[strum(to_string = "ASWF-Digital-Assets-1.1")]
    ASWF_Digital_Assets_1_1,
    #[serde(rename = "ASWF-Digital-Assets-1.0")]
    #[strum(to_string = "ASWF-Digital-Assets-1.0")]
    ASWF_Digital_Assets_1_0,
    #[serde(rename = "AAL")]
    #[strum(to_string = "AAL")]
    AAL,
    #[serde(rename = "Baekmuk")]
    #[strum(to_string = "Baekmuk")]
    Baekmuk,
    #[serde(rename = "Bahyph")]
    #[strum(to_string = "Bahyph")]
    Bahyph,
    #[serde(rename = "Barr")]
    #[strum(to_string = "Barr")]
    Barr,
    #[serde(rename = "Beerware")]
    #[strum(to_string = "Beerware")]
    Beerware,
    #[serde(rename = "Bitstream-Charter")]
    #[strum(to_string = "Bitstream-Charter")]
    Bitstream_Charter,
    #[serde(rename = "Bitstream-Vera")]
    #[strum(to_string = "Bitstream-Vera")]
    Bitstream_Vera,
    #[serde(rename = "BitTorrent-1.0")]
    #[strum(to_string = "BitTorrent-1.0")]
    BitTorrent_1_0,
    #[serde(rename = "BitTorrent-1.1")]
    #[strum(to_string = "BitTorrent-1.1")]
    BitTorrent_1_1,
    #[serde(rename = "BlueOak-1.0.0")]
    #[strum(to_string = "BlueOak-1.0.0")]
    BlueOak_1_0_0,
    #[serde(rename = "Boehm-GC")]
    #[strum(to_string = "Boehm-GC")]
    Boehm_GC,
    #[serde(rename = "BSL-1.0")]
    #[strum(to_string = "BSL-1.0")]
    BSL_1_0,
    #[serde(rename = "Borceux")]
    #[strum(to_string = "Borceux")]
    Borceux,
    #[serde(rename = "Brian-Gladman-3-Clause")]
    #[strum(to_string = "Brian-Gladman-3-Clause")]
    Brian_Gladman_3_Clause,
    #[serde(rename = "BSD-1-Clause")]
    #[strum(to_string = "BSD-1-Clause")]
    BSD_1_Clause,
    #[serde(rename = "BSD-2-Clause")]
    #[strum(to_string = "BSD-2-Clause")]
    BSD_2_Clause,
    #[serde(rename = "BSD-2-Clause-Views")]
    #[strum(to_string = "BSD-2-Clause-Views")]
    BSD_2_Clause_Views,
    #[serde(rename = "BSD-3-Clause")]
    #[strum(to_string = "BSD-3-Clause")]
    BSD_3_Clause,
    #[serde(rename = "BSD-3-Clause-Clear")]
    #[strum(to_string = "BSD-3-Clause-Clear")]
    BSD_3_Clause_Clear,
    #[serde(rename = "BSD-3-Clause-flex")]
    #[strum(to_string = "BSD-3-Clause-flex")]
    BSD_3_Clause_flex,
    #[serde(rename = "BSD-3-Clause-Modification")]
    #[strum(to_string = "BSD-3-Clause-Modification")]
    BSD_3_Clause_Modification,
    #[serde(rename = "BSD-3-Clause-No-Military-License")]
    #[strum(to_string = "BSD-3-Clause-No-Military-License")]
    BSD_3_Clause_No_Military_License,
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License")]
    #[strum(to_string = "BSD-3-Clause-No-Nuclear-License")]
    BSD_3_Clause_No_Nuclear_License,
    #[serde(rename = "BSD-3-Clause-No-Nuclear-License-2014")]
    #[strum(to_string = "BSD-3-Clause-No-Nuclear-License-2014")]
    BSD_3_Clause_No_Nuclear_License_2014,
    #[serde(rename = "BSD-3-Clause-No-Nuclear-Warranty")]
    #[strum(to_string = "BSD-3-Clause-No-Nuclear-Warranty")]
    BSD_3_Clause_No_Nuclear_Warranty,
    #[serde(rename = "BSD-3-Clause-Open-MPI")]
    #[strum(to_string = "BSD-3-Clause-Open-MPI")]
    BSD_3_Clause_Open_MPI,
    #[serde(rename = "BSD-3-Clause-Sun")]
    #[strum(to_string = "BSD-3-Clause-Sun")]
    BSD_3_Clause_Sun,
    #[serde(rename = "BSD-4-Clause-Shortened")]
    #[strum(to_string = "BSD-4-Clause-Shortened")]
    BSD_4_Clause_Shortened,
    #[serde(rename = "BSD-4-Clause")]
    #[strum(to_string = "BSD-4-Clause")]
    BSD_4_Clause,
    #[serde(rename = "BSD-4.3RENO")]
    #[strum(to_string = "BSD-4.3RENO")]
    BSD_4_3RENO,
    #[serde(rename = "BSD-4.3TAHOE")]
    #[strum(to_string = "BSD-4.3TAHOE")]
    BSD_4_3TAHOE,
    #[serde(rename = "BSD-Advertising-Acknowledgement")]
    #[strum(to_string = "BSD-Advertising-Acknowledgement")]
    BSD_Advertising_Acknowledgement,
    #[serde(rename = "BSD-Protection")]
    #[strum(to_string = "BSD-Protection")]
    BSD_Protection,
    #[serde(rename = "BSD-Source-Code")]
    #[strum(to_string = "BSD-Source-Code")]
    BSD_Source_Code,
    #[serde(rename = "BSD-3-Clause-Attribution")]
    #[strum(to_string = "BSD-3-Clause-Attribution")]
    BSD_3_Clause_Attribution,
    #[serde(rename = "BSD-Attribution-HPND-disclaimer")]
    #[strum(to_string = "BSD-Attribution-HPND-disclaimer")]
    BSD_Attribution_HPND_disclaimer,
    #[serde(rename = "0BSD")]
    #[strum(to_string = "0BSD")]
    Zero_BSD,
    #[serde(rename = "BSD-2-Clause-Patent")]
    #[strum(to_string = "BSD-2-Clause-Patent")]
    BSD_2_Clause_Patent,
    #[serde(rename = "BSD-4-Clause-UC")]
    #[strum(to_string = "BSD-4-Clause-UC")]
    BSD_4_Clause_UC,
    #[serde(rename = "BSD-Inferno-Nettverk")]
    #[strum(to_string = "BSD-Inferno-Nettverk")]
    BSD_Inferno_Nettverk,
    #[serde(rename = "BUSL-1.1")]
    #[strum(to_string = "BUSL-1.1")]
    BUSL_1_1,
    #[serde(rename = "bzip2-1.0.6")]
    #[strum(to_string = "bzip2-1.0.6")]
    bzip2_1_0_6,
    #[serde(rename = "Caldera")]
    #[strum(to_string = "Caldera")]
    Caldera,
    #[serde(rename = "CECILL-1.0")]
    #[strum(to_string = "CECILL-1.0")]
    CECILL_1_0,
    #[serde(rename = "CECILL-1.1")]
    #[strum(to_string = "CECILL-1.1")]
    CECILL_1_1,
    #[serde(rename = "CECILL-2.0")]
    #[strum(to_string = "CECILL-2.0")]
    CECILL_2_0,
    #[serde(rename = "CECILL-2.1")]
    #[strum(to_string = "CECILL-2.1")]
    CECILL_2_1,
    #[serde(rename = "CECILL-B")]
    #[strum(to_string = "CECILL-B")]
    CECILL_B,
    #[serde(rename = "CECILL-C")]
    #[strum(to_string = "CECILL-C")]
    CECILL_C,
    #[serde(rename = "CERN-OHL-1.1")]
    #[strum(to_string = "CERN-OHL-1.1")]
    CERN_OHL_1_1,
    #[serde(rename = "CERN-OHL-1.2")]
    #[strum(to_string = "CERN-OHL-1.2")]
    CERN_OHL_1_2,
    #[serde(rename = "CERN-OHL-P-2.0")]
    #[strum(to_string = "CERN-OHL-P-2.0")]
    CERN_OHL_P_2_0,
    #[serde(rename = "CERN-OHL-S-2.0")]
    #[strum(to_string = "CERN-OHL-S-2.0")]
    CERN_OHL_S_2_0,
    #[serde(rename = "CERN-OHL-W-2.0")]
    #[strum(to_string = "CERN-OHL-W-2.0")]
    CERN_OHL_W_2_0,
    #[serde(rename = "CFITSIO")]
    #[strum(to_string = "CFITSIO")]
    CFITSIO,
    #[serde(rename = "check-cvs")]
    #[strum(to_string = "check-cvs")]
    check_cvs,
    #[serde(rename = "checkmk")]
    #[strum(to_string = "checkmk")]
    checkmk,
    #[serde(rename = "ClArtistic")]
    #[strum(to_string = "ClArtistic")]
    ClArtistic,
    #[serde(rename = "Clips")]
    #[strum(to_string = "Clips")]
    Clips,
    #[serde(rename = "MIT-CMU")]
    #[strum(to_string = "MIT-CMU")]
    MIT_CMU,
    #[serde(rename = "CMU-Mach")]
    #[strum(to_string = "CMU-Mach")]
    CMU_Mach,
    #[serde(rename = "CNRI-Jython")]
    #[strum(to_string = "CNRI-Jython")]
    CNRI_Jython,
    #[serde(rename = "CNRI-Python")]
    #[strum(to_string = "CNRI-Python")]
    CNRI_Python,
    #[serde(rename = "CNRI-Python-GPL-Compatible")]
    #[strum(to_string = "CNRI-Python-GPL-Compatible")]
    CNRI_Python_GPL_Compatible,
    #[serde(rename = "CPOL-1.02")]
    #[strum(to_string = "CPOL-1.02")]
    CPOL_1_02,
    #[serde(rename = "CDDL-1.0")]
    #[strum(to_string = "CDDL-1.0")]
    CDDL_1_0,
    #[serde(rename = "CDDL-1.1")]
    #[strum(to_string = "CDDL-1.1")]
    CDDL_1_1,
    #[serde(rename = "CDL-1.0")]
    #[strum(to_string = "CDL-1.0")]
    CDL_1_0,
    #[serde(rename = "LOOP")]
    #[strum(to_string = "LOOP")]
    LOOP,
    #[serde(rename = "CPAL-1.0")]
    #[strum(to_string = "CPAL-1.0")]
    CPAL_1_0,
    #[serde(rename = "CPL-1.0")]
    #[strum(to_string = "CPL-1.0")]
    CPL_1_0,
    #[serde(rename = "CDLA-Permissive-1.0")]
    #[strum(to_string = "CDLA-Permissive-1.0")]
    CDLA_Permissive_1_0,
    #[serde(rename = "CDLA-Permissive-2.0")]
    #[strum(to_string = "CDLA-Permissive-2.0")]
    CDLA_Permissive_2_0,
    #[serde(rename = "CDLA-Sharing-1.0")]
    #[strum(to_string = "CDLA-Sharing-1.0")]
    CDLA_Sharing_1_0,
    #[serde(rename = "Community-Spec-1.0")]
    #[strum(to_string = "Community-Spec-1.0")]
    Community_Spec_1_0,
    #[serde(rename = "C-UDA-1.0")]
    #[strum(to_string = "C-UDA-1.0")]
    C_UDA_1_0,
    #[serde(rename = "CATOSL-1.1")]
    #[strum(to_string = "CATOSL-1.1")]
    CATOSL_1_1,
    #[serde(rename = "Condor-1.1")]
    #[strum(to_string = "Condor-1.1")]
    Condor_1_1,
    #[serde(rename = "COIL-1.0")]
    #[strum(to_string = "COIL-1.0")]
    COIL_1_0,
    #[serde(rename = "copyleft-next-0.3.0")]
    #[strum(to_string = "copyleft-next-0.3.0")]
    copyleft_next_0_3_0,
    #[serde(rename = "copyleft-next-0.3.1")]
    #[strum(to_string = "copyleft-next-0.3.1")]
    copyleft_next_0_3_1,
    #[serde(rename = "Cornell-Lossless-JPEG")]
    #[strum(to_string = "Cornell-Lossless-JPEG")]
    Cornell_Lossless_JPEG,
    #[serde(rename = "CC-BY-1.0")]
    #[strum(to_string = "CC-BY-1.0")]
    CC_BY_1_0,
    #[serde(rename = "CC-BY-2.0")]
    #[strum(to_string = "CC-BY-2.0")]
    CC_BY_2_0,
    #[serde(rename = "CC-BY-2.5-AU")]
    #[strum(to_string = "CC-BY-2.5-AU")]
    CC_BY_2_5_AU,
    #[serde(rename = "CC-BY-2.5")]
    #[strum(to_string = "CC-BY-2.5")]
    CC_BY_2_5,
    #[serde(rename = "CC-BY-3.0-AT")]
    #[strum(to_string = "CC-BY-3.0-AT")]
    CC_BY_3_0_AT,
    #[serde(rename = "CC-BY-3.0-DE")]
    #[strum(to_string = "CC-BY-3.0-DE")]
    CC_BY_3_0_DE,
    #[serde(rename = "CC-BY-3.0-IGO")]
    #[strum(to_string = "CC-BY-3.0-IGO")]
    CC_BY_3_0_IGO,
    #[serde(rename = "CC-BY-3.0-NL")]
    #[strum(to_string = "CC-BY-3.0-NL")]
    CC_BY_3_0_NL,
    #[serde(rename = "CC-BY-3.0-US")]
    #[strum(to_string = "CC-BY-3.0-US")]
    CC_BY_3_0_US,
    #[serde(rename = "CC-BY-3.0")]
    #[strum(to_string = "CC-BY-3.0")]
    CC_BY_3_0,
    #[serde(rename = "CC-BY-4.0")]
    #[strum(to_string = "CC-BY-4.0")]
    CC_BY_4_0,
    #[serde(rename = "CC-BY-ND-1.0")]
    #[strum(to_string = "CC-BY-ND-1.0")]
    CC_BY_ND_1_0,
    #[serde(rename = "CC-BY-ND-2.0")]
    #[strum(to_string = "CC-BY-ND-2.0")]
    CC_BY_ND_2_0,
    #[serde(rename = "CC-BY-ND-2.5")]
    #[strum(to_string = "CC-BY-ND-2.5")]
    CC_BY_ND_2_5,
    #[serde(rename = "CC-BY-ND-3.0-DE")]
    #[strum(to_string = "CC-BY-ND-3.0-DE")]
    CC_BY_ND_3_0_DE,
    #[serde(rename = "CC-BY-ND-3.0")]
    #[strum(to_string = "CC-BY-ND-3.0")]
    CC_BY_ND_3_0,
    #[serde(rename = "CC-BY-ND-4.0")]
    #[strum(to_string = "CC-BY-ND-4.0")]
    CC_BY_ND_4_0,
    #[serde(rename = "CC-BY-NC-1.0")]
    #[strum(to_string = "CC-BY-NC-1.0")]
    CC_BY_NC_1_0,
    #[serde(rename = "CC-BY-NC-2.0")]
    #[strum(to_string = "CC-BY-NC-2.0")]
    CC_BY_NC_2_0,
    #[serde(rename = "CC-BY-NC-2.5")]
    #[strum(to_string = "CC-BY-NC-2.5")]
    CC_BY_NC_2_5,
    #[serde(rename = "CC-BY-NC-3.0-DE")]
    #[strum(to_string = "CC-BY-NC-3.0-DE")]
    CC_BY_NC_3_0_DE,
    #[serde(rename = "CC-BY-NC-3.0")]
    #[strum(to_string = "CC-BY-NC-3.0")]
    CC_BY_NC_3_0,
    #[serde(rename = "CC-BY-NC-4.0")]
    #[strum(to_string = "CC-BY-NC-4.0")]
    CC_BY_NC_4_0,
    #[serde(rename = "CC-BY-NC-ND-1.0")]
    #[strum(to_string = "CC-BY-NC-ND-1.0")]
    CC_BY_NC_ND_1_0,
    #[serde(rename = "CC-BY-NC-ND-2.0")]
    #[strum(to_string = "CC-BY-NC-ND-2.0")]
    CC_BY_NC_ND_2_0,
    #[serde(rename = "CC-BY-NC-ND-2.5")]
    #[strum(to_string = "CC-BY-NC-ND-2.5")]
    CC_BY_NC_ND_2_5,
    #[serde(rename = "CC-BY-NC-ND-3.0-DE")]
    #[strum(to_string = "CC-BY-NC-ND-3.0-DE")]
    CC_BY_NC_ND_3_0_DE,
    #[serde(rename = "CC-BY-NC-ND-3.0-IGO")]
    #[strum(to_string = "CC-BY-NC-ND-3.0-IGO")]
    CC_BY_NC_ND_3_0_IGO,
    #[serde(rename = "CC-BY-NC-ND-3.0")]
    #[strum(to_string = "CC-BY-NC-ND-3.0")]
    CC_BY_NC_ND_3_0,
    #[serde(rename = "CC-BY-NC-ND-4.0")]
    #[strum(to_string = "CC-BY-NC-ND-4.0")]
    CC_BY_NC_ND_4_0,
    #[serde(rename = "CC-BY-NC-SA-1.0")]
    #[strum(to_string = "CC-BY-NC-SA-1.0")]
    CC_BY_NC_SA_1_0,
    #[serde(rename = "CC-BY-NC-SA-2.0-UK")]
    #[strum(to_string = "CC-BY-NC-SA-2.0-UK")]
    CC_BY_NC_SA_2_0_UK,
    #[serde(rename = "CC-BY-NC-SA-2.0")]
    #[strum(to_string = "CC-BY-NC-SA-2.0")]
    CC_BY_NC_SA_2_0,
    #[serde(rename = "CC-BY-NC-SA-2.0-DE")]
    #[strum(to_string = "CC-BY-NC-SA-2.0-DE")]
    CC_BY_NC_SA_2_0_DE,
    #[serde(rename = "CC-BY-NC-SA-2.5")]
    #[strum(to_string = "CC-BY-NC-SA-2.5")]
    CC_BY_NC_SA_2_5,
    #[serde(rename = "CC-BY-NC-SA-3.0-DE")]
    #[strum(to_string = "CC-BY-NC-SA-3.0-DE")]
    CC_BY_NC_SA_3_0_DE,
    #[serde(rename = "CC-BY-NC-SA-3.0-IGO")]
    #[strum(to_string = "CC-BY-NC-SA-3.0-IGO")]
    CC_BY_NC_SA_3_0_IGO,
    #[serde(rename = "CC-BY-NC-SA-3.0")]
    #[strum(to_string = "CC-BY-NC-SA-3.0")]
    CC_BY_NC_SA_3_0,
    #[serde(rename = "CC-BY-NC-SA-4.0")]
    #[strum(to_string = "CC-BY-NC-SA-4.0")]
    CC_BY_NC_SA_4_0,
    #[serde(rename = "CC-BY-SA-1.0")]
    #[strum(to_string = "CC-BY-SA-1.0")]
    CC_BY_SA_1_0,
    #[serde(rename = "CC-BY-SA-2.0-UK")]
    #[strum(to_string = "CC-BY-SA-2.0-UK")]
    CC_BY_SA_2_0_UK,
    #[serde(rename = "CC-BY-SA-2.0")]
    #[strum(to_string = "CC-BY-SA-2.0")]
    CC_BY_SA_2_0,
    #[serde(rename = "CC-BY-SA-2.1-JP")]
    #[strum(to_string = "CC-BY-SA-2.1-JP")]
    CC_BY_SA_2_1_JP,
    #[serde(rename = "CC-BY-SA-2.5")]
    #[strum(to_string = "CC-BY-SA-2.5")]
    CC_BY_SA_2_5,
    #[serde(rename = "CC-BY-SA-3.0-AT")]
    #[strum(to_string = "CC-BY-SA-3.0-AT")]
    CC_BY_SA_3_0_AT,
    #[serde(rename = "CC-BY-SA-3.0-DE")]
    #[strum(to_string = "CC-BY-SA-3.0-DE")]
    CC_BY_SA_3_0_DE,
    #[serde(rename = "CC-BY-SA-3.0")]
    #[strum(to_string = "CC-BY-SA-3.0")]
    CC_BY_SA_3_0,
    #[serde(rename = "CC-BY-SA-4.0")]
    #[strum(to_string = "CC-BY-SA-4.0")]
    CC_BY_SA_4_0,
    #[serde(rename = "CC-BY-NC-SA-2.0-FR")]
    #[strum(to_string = "CC-BY-NC-SA-2.0-FR")]
    CC_BY_NC_SA_2_0_FR,
    #[serde(rename = "CC-BY-SA-3.0-IGO")]
    #[strum(to_string = "CC-BY-SA-3.0-IGO")]
    CC_BY_SA_3_0_IGO,
    #[serde(rename = "CC-PDDC")]
    #[strum(to_string = "CC-PDDC")]
    CC_PDDC,
    #[serde(rename = "CC0-1.0")]
    #[strum(to_string = "CC0-1.0")]
    CC0_1_0,
    #[serde(rename = "Cronyx")]
    #[strum(to_string = "Cronyx")]
    Cronyx,
    #[serde(rename = "Crossword")]
    #[strum(to_string = "Crossword")]
    Crossword,
    #[serde(rename = "CAL-1.0")]
    #[strum(to_string = "CAL-1.0")]
    CAL_1_0,
    #[serde(rename = "CAL-1.0-Combined-Work-Exception")]
    #[strum(to_string = "CAL-1.0-Combined-Work-Exception")]
    CAL_1_0_Combined_Work_Exception,
    #[serde(rename = "CrystalStacker")]
    #[strum(to_string = "CrystalStacker")]
    CrystalStacker,
    #[serde(rename = "CUA-OPL-1.0")]
    #[strum(to_string = "CUA-OPL-1.0")]
    CUA_OPL_1_0,
    #[serde(rename = "Cube")]
    #[strum(to_string = "Cube")]
    Cube,
    #[serde(rename = "curl")]
    #[strum(to_string = "curl")]
    curl,
    #[serde(rename = "DL-DE-BY-2.0")]
    #[strum(to_string = "DL-DE-BY-2.0")]
    DL_DE_BY_2_0,
    #[serde(rename = "DL-DE-ZERO-2.0")]
    #[strum(to_string = "DL-DE-ZERO-2.0")]
    DL_DE_ZERO_2_0,
    #[serde(rename = "dtoa")]
    #[strum(to_string = "dtoa")]
    dtoa,
    #[serde(rename = "DRL-1.0")]
    #[strum(to_string = "DRL-1.0")]
    DRL_1_0,
    #[serde(rename = "D-FSL-1.0")]
    #[strum(to_string = "D-FSL-1.0")]
    D_FSL_1_0,
    #[serde(rename = "diffmark")]
    #[strum(to_string = "diffmark")]
    diffmark,
    #[serde(rename = "WTFPL")]
    #[strum(to_string = "WTFPL")]
    WTFPL,
    #[serde(rename = "DOC")]
    #[strum(to_string = "DOC")]
    DOC,
    #[serde(rename = "Dotseqn")]
    #[strum(to_string = "Dotseqn")]
    Dotseqn,
    #[serde(rename = "DSDP")]
    #[strum(to_string = "DSDP")]
    DSDP,
    #[serde(rename = "dvipdfm")]
    #[strum(to_string = "dvipdfm")]
    dvipdfm,
    #[serde(rename = "EPL-1.0")]
    #[strum(to_string = "EPL-1.0")]
    EPL_1_0,
    #[serde(rename = "EPL-2.0")]
    #[strum(to_string = "EPL-2.0")]
    EPL_2_0,
    #[serde(rename = "ECL-1.0")]
    #[strum(to_string = "ECL-1.0")]
    ECL_1_0,
    #[serde(rename = "ECL-2.0")]
    #[strum(to_string = "ECL-2.0")]
    ECL_2_0,
    #[serde(rename = "eGenix")]
    #[strum(to_string = "eGenix")]
    eGenix,
    #[serde(rename = "EFL-1.0")]
    #[strum(to_string = "EFL-1.0")]
    EFL_1_0,
    #[serde(rename = "EFL-2.0")]
    #[strum(to_string = "EFL-2.0")]
    EFL_2_0,
    #[serde(rename = "Elastic-2.0")]
    #[strum(to_string = "Elastic-2.0")]
    Elastic_2_0,
    #[serde(rename = "MIT-advertising")]
    #[strum(to_string = "MIT-advertising")]
    MIT_advertising,
    #[serde(rename = "MIT-enna")]
    #[strum(to_string = "MIT-enna")]
    MIT_enna,
    #[serde(rename = "Entessa")]
    #[strum(to_string = "Entessa")]
    Entessa,
    #[serde(rename = "EPICS")]
    #[strum(to_string = "EPICS")]
    EPICS,
    #[serde(rename = "ErlPL-1.1")]
    #[strum(to_string = "ErlPL-1.1")]
    ErlPL_1_1,
    #[serde(rename = "etalab-2.0")]
    #[strum(to_string = "etalab-2.0")]
    etalab_2_0,
    #[serde(rename = "EUDatagrid")]
    #[strum(to_string = "EUDatagrid")]
    EUDatagrid,
    #[serde(rename = "EUPL-1.0")]
    #[strum(to_string = "EUPL-1.0")]
    EUPL_1_0,
    #[serde(rename = "EUPL-1.1")]
    #[strum(to_string = "EUPL-1.1")]
    EUPL_1_1,
    #[serde(rename = "EUPL-1.2")]
    #[strum(to_string = "EUPL-1.2")]
    EUPL_1_2,
    #[serde(rename = "Eurosym")]
    #[strum(to_string = "Eurosym")]
    Eurosym,
    #[serde(rename = "Fair")]
    #[strum(to_string = "Fair")]
    Fair,
    #[serde(rename = "MIT-feh")]
    #[strum(to_string = "MIT-feh")]
    MIT_feh,
    #[serde(rename = "Ferguson-Twofish")]
    #[strum(to_string = "Ferguson-Twofish")]
    Ferguson_Twofish,
    #[serde(rename = "Frameworx-1.0")]
    #[strum(to_string = "Frameworx-1.0")]
    Frameworx_1_0,
    #[serde(rename = "FDK-AAC")]
    #[strum(to_string = "FDK-AAC")]
    FDK_AAC,
    #[serde(rename = "FreeBSD-DOC")]
    #[strum(to_string = "FreeBSD-DOC")]
    FreeBSD_DOC,
    #[serde(rename = "FreeImage")]
    #[strum(to_string = "FreeImage")]
    FreeImage,
    #[serde(rename = "FTL")]
    #[strum(to_string = "FTL")]
    FTL,
    #[serde(rename = "FSFAP")]
    #[strum(to_string = "FSFAP")]
    FSFAP,
    #[serde(rename = "FSFUL")]
    #[strum(to_string = "FSFUL")]
    FSFUL,
    #[serde(rename = "FSFULLRWD")]
    #[strum(to_string = "FSFULLRWD")]
    FSFULLRWD,
    #[serde(rename = "FSFULLR")]
    #[strum(to_string = "FSFULLR")]
    FSFULLR,
    #[serde(rename = "Furuseth")]
    #[strum(to_string = "Furuseth")]
    Furuseth,
    #[serde(rename = "FBM")]
    #[strum(to_string = "FBM")]
    FBM,
    #[serde(rename = "fwlw")]
    #[strum(to_string = "fwlw")]
    fwlw,
    #[serde(rename = "GD")]
    #[strum(to_string = "GD")]
    GD,
    #[serde(rename = "Giftware")]
    #[strum(to_string = "Giftware")]
    Giftware,
    #[serde(rename = "GL2PS")]
    #[strum(to_string = "GL2PS")]
    GL2PS,
    #[serde(rename = "Glulxe")]
    #[strum(to_string = "Glulxe")]
    Glulxe,
    #[serde(rename = "AGPL-3.0-only")]
    #[strum(to_string = "AGPL-3.0-only")]
    AGPL_3_0_only,
    #[serde(rename = "AGPL-3.0-or-later")]
    #[strum(to_string = "AGPL-3.0-or-later")]
    AGPL_3_0_or_later,
    #[serde(rename = "GFDL-1.1-only")]
    #[strum(to_string = "GFDL-1.1-only")]
    GFDL_1_1_only,
    #[serde(rename = "GFDL-1.1-invariants-only")]
    #[strum(to_string = "GFDL-1.1-invariants-only")]
    GFDL_1_1_invariants_only,
    #[serde(rename = "GFDL-1.1-no-invariants-only")]
    #[strum(to_string = "GFDL-1.1-no-invariants-only")]
    GFDL_1_1_no_invariants_only,
    #[serde(rename = "GFDL-1.1-or-later")]
    #[strum(to_string = "GFDL-1.1-or-later")]
    GFDL_1_1_or_later,
    #[serde(rename = "GFDL-1.1-invariants-or-later")]
    #[strum(to_string = "GFDL-1.1-invariants-or-later")]
    GFDL_1_1_invariants_or_later,
    #[serde(rename = "GFDL-1.1-no-invariants-or-later")]
    #[strum(to_string = "GFDL-1.1-no-invariants-or-later")]
    GFDL_1_1_no_invariants_or_later,
    #[serde(rename = "GFDL-1.2-only")]
    #[strum(to_string = "GFDL-1.2-only")]
    GFDL_1_2_only,
    #[serde(rename = "GFDL-1.2-invariants-only")]
    #[strum(to_string = "GFDL-1.2-invariants-only")]
    GFDL_1_2_invariants_only,
    #[serde(rename = "GFDL-1.2-no-invariants-only")]
    #[strum(to_string = "GFDL-1.2-no-invariants-only")]
    GFDL_1_2_no_invariants_only,
    #[serde(rename = "GFDL-1.2-or-later")]
    #[strum(to_string = "GFDL-1.2-or-later")]
    GFDL_1_2_or_later,
    #[serde(rename = "GFDL-1.2-invariants-or-later")]
    #[strum(to_string = "GFDL-1.2-invariants-or-later")]
    GFDL_1_2_invariants_or_later,
    #[serde(rename = "GFDL-1.2-no-invariants-or-later")]
    #[strum(to_string = "GFDL-1.2-no-invariants-or-later")]
    GFDL_1_2_no_invariants_or_later,
    #[serde(rename = "GFDL-1.3-only")]
    #[strum(to_string = "GFDL-1.3-only")]
    GFDL_1_3_only,
    #[serde(rename = "GFDL-1.3-invariants-only")]
    #[strum(to_string = "GFDL-1.3-invariants-only")]
    GFDL_1_3_invariants_only,
    #[serde(rename = "GFDL-1.3-no-invariants-only")]
    #[strum(to_string = "GFDL-1.3-no-invariants-only")]
    GFDL_1_3_no_invariants_only,
    #[serde(rename = "GFDL-1.3-or-later")]
    #[strum(to_string = "GFDL-1.3-or-later")]
    GFDL_1_3_or_later,
    #[serde(rename = "GFDL-1.3-invariants-or-later")]
    #[strum(to_string = "GFDL-1.3-invariants-or-later")]
    GFDL_1_3_invariants_or_later,
    #[serde(rename = "GFDL-1.3-no-invariants-or-later")]
    #[strum(to_string = "GFDL-1.3-no-invariants-or-later")]
    GFDL_1_3_no_invariants_or_later,
    #[serde(rename = "GPL-1.0-only")]
    #[strum(to_string = "GPL-1.0-only")]
    GPL_1_0_only,
    #[serde(rename = "GPL-1.0-or-later")]
    #[strum(to_string = "GPL-1.0-or-later")]
    GPL_1_0_or_later,
    #[serde(rename = "GPL-2.0-only")]
    #[strum(to_string = "GPL-2.0-only")]
    GPL_2_0_only,
    #[serde(rename = "GPL-2.0-or-later")]
    #[strum(to_string = "GPL-2.0-or-later")]
    GPL_2_0_or_later,
    #[serde(rename = "GPL-3.0-only")]
    #[strum(to_string = "GPL-3.0-only")]
    GPL_3_0_only,
    #[serde(rename = "GPL-3.0-or-later")]
    #[strum(to_string = "GPL-3.0-or-later")]
    GPL_3_0_or_later,
    #[serde(rename = "LGPL-2.1-only")]
    #[strum(to_string = "LGPL-2.1-only")]
    LGPL_2_1_only,
    #[serde(rename = "LGPL-2.1-or-later")]
    #[strum(to_string = "LGPL-2.1-or-later")]
    LGPL_2_1_or_later,
    #[serde(rename = "LGPL-3.0-only")]
    #[strum(to_string = "LGPL-3.0-only")]
    LGPL_3_0_only,
    #[serde(rename = "LGPL-3.0-or-later")]
    #[strum(to_string = "LGPL-3.0-or-later")]
    LGPL_3_0_or_later,
    #[serde(rename = "LGPL-2.0-only")]
    #[strum(to_string = "LGPL-2.0-only")]
    LGPL_2_0_only,
    #[serde(rename = "LGPL-2.0-or-later")]
    #[strum(to_string = "LGPL-2.0-or-later")]
    LGPL_2_0_or_later,
    #[serde(rename = "gnuplot")]
    #[strum(to_string = "gnuplot")]
    gnuplot,
    #[serde(rename = "GLWTPL")]
    #[strum(to_string = "GLWTPL")]
    GLWTPL,
    #[serde(rename = "Graphics-Gems")]
    #[strum(to_string = "Graphics-Gems")]
    Graphics_Gems,
    #[serde(rename = "gSOAP-1.3b")]
    #[strum(to_string = "gSOAP-1.3b")]
    gSOAP_1_3b,
    #[serde(rename = "HaskellReport")]
    #[strum(to_string = "HaskellReport")]
    HaskellReport,
    #[serde(rename = "HP-1986")]
    #[strum(to_string = "HP-1986")]
    HP_1986,
    #[serde(rename = "HP-1989")]
    #[strum(to_string = "HP-1989")]
    HP_1989,
    #[serde(rename = "BSD-3-Clause-HP")]
    #[strum(to_string = "BSD-3-Clause-HP")]
    BSD_3_Clause_HP,
    #[serde(rename = "Hippocratic-2.1")]
    #[strum(to_string = "Hippocratic-2.1")]
    Hippocratic_2_1,
    #[serde(rename = "HPND")]
    #[strum(to_string = "HPND")]
    HPND,
    #[serde(rename = "HPND-DEC")]
    #[strum(to_string = "HPND-DEC")]
    HPND_DEC,
    #[serde(rename = "HPND-doc-sell")]
    #[strum(to_string = "HPND-doc-sell")]
    HPND_doc_sell,
    #[serde(rename = "HPND-doc")]
    #[strum(to_string = "HPND-doc")]
    HPND_doc,
    #[serde(rename = "HPND-Markus-Kuhn")]
    #[strum(to_string = "HPND-Markus-Kuhn")]
    HPND_Markus_Kuhn,
    #[serde(rename = "HPND-Pbmplus")]
    #[strum(to_string = "HPND-Pbmplus")]
    HPND_Pbmplus,
    #[serde(rename = "HPND-sell-regexpr")]
    #[strum(to_string = "HPND-sell-regexpr")]
    HPND_sell_regexpr,
    #[serde(rename = "HPND-sell-variant")]
    #[strum(to_string = "HPND-sell-variant")]
    HPND_sell_variant,
    #[serde(rename = "HPND-UC")]
    #[strum(to_string = "HPND-UC")]
    HPND_UC,
    #[serde(rename = "HPND-sell-variant-MIT-disclaimer")]
    #[strum(to_string = "HPND-sell-variant-MIT-disclaimer")]
    HPND_sell_variant_MIT_disclaimer,
    #[serde(rename = "HPND-export-US")]
    #[strum(to_string = "HPND-export-US")]
    HPND_export_US,
    #[serde(rename = "HPND-export-US-modify")]
    #[strum(to_string = "HPND-export-US-modify")]
    HPND_export_US_modify,
    #[serde(rename = "HTMLTIDY")]
    #[strum(to_string = "HTMLTIDY")]
    HTMLTIDY,
    #[serde(rename = "IBM-pibs")]
    #[strum(to_string = "IBM-pibs")]
    IBM_pibs,
    #[serde(rename = "IPL-1.0")]
    #[strum(to_string = "IPL-1.0")]
    IPL_1_0,
    #[serde(rename = "ICU")]
    #[strum(to_string = "ICU")]
    ICU,
    #[serde(rename = "IEC-Code-Components-EULA")]
    #[strum(to_string = "IEC-Code-Components-EULA")]
    IEC_Code_Components_EULA,
    #[serde(rename = "ImageMagick")]
    #[strum(to_string = "ImageMagick")]
    ImageMagick,
    #[serde(rename = "iMatix")]
    #[strum(to_string = "iMatix")]
    iMatix,
    #[serde(rename = "Imlib2")]
    #[strum(to_string = "Imlib2")]
    Imlib2,
    #[serde(rename = "IJG")]
    #[strum(to_string = "IJG")]
    IJG,
    #[serde(rename = "IJG-short")]
    #[strum(to_string = "IJG-short")]
    IJG_short,
    #[serde(rename = "Info-ZIP")]
    #[strum(to_string = "Info-ZIP")]
    Info_ZIP,
    #[serde(rename = "Inner-Net-2.0")]
    #[strum(to_string = "Inner-Net-2.0")]
    Inner_Net_2_0,
    #[serde(rename = "Intel-ACPI")]
    #[strum(to_string = "Intel-ACPI")]
    Intel_ACPI,
    #[serde(rename = "Intel")]
    #[strum(to_string = "Intel")]
    Intel,
    #[serde(rename = "Interbase-1.0")]
    #[strum(to_string = "Interbase-1.0")]
    Interbase_1_0,
    #[serde(rename = "IPA")]
    #[strum(to_string = "IPA")]
    IPA,
    #[serde(rename = "ISC")]
    #[strum(to_string = "ISC")]
    ISC,
    #[serde(rename = "Jam")]
    #[strum(to_string = "Jam")]
    Jam,
    #[serde(rename = "JPNIC")]
    #[strum(to_string = "JPNIC")]
    JPNIC,
    #[serde(rename = "JasPer-2.0")]
    #[strum(to_string = "JasPer-2.0")]
    JasPer_2_0,
    #[serde(rename = "JPL-image")]
    #[strum(to_string = "JPL-image")]
    JPL_image,
    #[serde(rename = "JSON")]
    #[strum(to_string = "JSON")]
    JSON,
    #[serde(rename = "Kastrup")]
    #[strum(to_string = "Kastrup")]
    Kastrup,
    #[serde(rename = "Kazlib")]
    #[strum(to_string = "Kazlib")]
    Kazlib,
    #[serde(rename = "Knuth-CTAN")]
    #[strum(to_string = "Knuth-CTAN")]
    Knuth_CTAN,
    #[serde(rename = "LPPL-1.0")]
    #[strum(to_string = "LPPL-1.0")]
    LPPL_1_0,
    #[serde(rename = "LPPL-1.1")]
    #[strum(to_string = "LPPL-1.1")]
    LPPL_1_1,
    #[serde(rename = "LPPL-1.2")]
    #[strum(to_string = "LPPL-1.2")]
    LPPL_1_2,
    #[serde(rename = "LPPL-1.3a")]
    #[strum(to_string = "LPPL-1.3a")]
    LPPL_1_3a,
    #[serde(rename = "LPPL-1.3c")]
    #[strum(to_string = "LPPL-1.3c")]
    LPPL_1_3c,
    #[serde(rename = "Latex2e")]
    #[strum(to_string = "Latex2e")]
    Latex2e,
    #[serde(rename = "Latex2e-translated-notice")]
    #[strum(to_string = "Latex2e-translated-notice")]
    Latex2e_translated_notice,
    #[serde(rename = "BSD-3-Clause-LBNL")]
    #[strum(to_string = "BSD-3-Clause-LBNL")]
    BSD_3_Clause_LBNL,
    #[serde(rename = "Leptonica")]
    #[strum(to_string = "Leptonica")]
    Leptonica,
    #[serde(rename = "LGPLLR")]
    #[strum(to_string = "LGPLLR")]
    LGPLLR,
    #[serde(rename = "Libpng")]
    #[strum(to_string = "Libpng")]
    Libpng,
    #[serde(rename = "libselinux-1.0")]
    #[strum(to_string = "libselinux-1.0")]
    libselinux_1_0,
    #[serde(rename = "libtiff")]
    #[strum(to_string = "libtiff")]
    libtiff,
    #[serde(rename = "libutil-David-Nugent")]
    #[strum(to_string = "libutil-David-Nugent")]
    libutil_David_Nugent,
    #[serde(rename = "LAL-1.2")]
    #[strum(to_string = "LAL-1.2")]
    LAL_1_2,
    #[serde(rename = "LAL-1.3")]
    #[strum(to_string = "LAL-1.3")]
    LAL_1_3,
    #[serde(rename = "LiLiQ-P-1.1")]
    #[strum(to_string = "LiLiQ-P-1.1")]
    LiLiQ_P_1_1,
    #[serde(rename = "LiLiQ-Rplus-1.1")]
    #[strum(to_string = "LiLiQ-Rplus-1.1")]
    LiLiQ_Rplus_1_1,
    #[serde(rename = "LiLiQ-R-1.1")]
    #[strum(to_string = "LiLiQ-R-1.1")]
    LiLiQ_R_1_1,
    #[serde(rename = "Linux-OpenIB")]
    #[strum(to_string = "Linux-OpenIB")]
    Linux_OpenIB,
    #[serde(rename = "Linux-man-pages-1-para")]
    #[strum(to_string = "Linux-man-pages-1-para")]
    Linux_man_pages_1_para,
    #[serde(rename = "Linux-man-pages-copyleft")]
    #[strum(to_string = "Linux-man-pages-copyleft")]
    Linux_man_pages_copyleft,
    #[serde(rename = "Linux-man-pages-copyleft-2-para")]
    #[strum(to_string = "Linux-man-pages-copyleft-2-para")]
    Linux_man_pages_copyleft_2_para,
    #[serde(rename = "Linux-man-pages-copyleft-var")]
    #[strum(to_string = "Linux-man-pages-copyleft-var")]
    Linux_man_pages_copyleft_var,
    #[serde(rename = "lsof")]
    #[strum(to_string = "lsof")]
    lsof,
    #[serde(rename = "LPL-1.02")]
    #[strum(to_string = "LPL-1.02")]
    LPL_1_02,
    #[serde(rename = "LPL-1.0")]
    #[strum(to_string = "LPL-1.0")]
    LPL_1_0,
    #[serde(rename = "Lucida-Bitmap-Fonts")]
    #[strum(to_string = "Lucida-Bitmap-Fonts")]
    Lucida_Bitmap_Fonts,
    #[serde(rename = "LZMA-SDK-9.11-to-9.20")]
    #[strum(to_string = "LZMA-SDK-9.11-to-9.20")]
    LZMA_SDK_9_11_to_9_20,
    #[serde(rename = "LZMA-SDK-9.22")]
    #[strum(to_string = "LZMA-SDK-9.22")]
    LZMA_SDK_9_22,
    #[serde(rename = "magaz")]
    #[strum(to_string = "magaz")]
    magaz,
    #[serde(rename = "MakeIndex")]
    #[strum(to_string = "MakeIndex")]
    MakeIndex,
    #[serde(rename = "Martin-Birgmeier")]
    #[strum(to_string = "Martin-Birgmeier")]
    Martin_Birgmeier,
    #[serde(rename = "MTLL")]
    #[strum(to_string = "MTLL")]
    MTLL,
    #[serde(rename = "McPhee-slideshow")]
    #[strum(to_string = "McPhee-slideshow")]
    McPhee_slideshow,
    #[serde(rename = "metamail")]
    #[strum(to_string = "metamail")]
    metamail,
    #[serde(rename = "MS-LPL")]
    #[strum(to_string = "MS-LPL")]
    MS_LPL,
    #[serde(rename = "MS-PL")]
    #[strum(to_string = "MS-PL")]
    MS_PL,
    #[serde(rename = "MS-RL")]
    #[strum(to_string = "MS-RL")]
    MS_RL,
    #[serde(rename = "Minpack")]
    #[strum(to_string = "Minpack")]
    Minpack,
    #[serde(rename = "MITNFA")]
    #[strum(to_string = "MITNFA")]
    MITNFA,
    #[serde(rename = "MIT-Festival")]
    #[strum(to_string = "MIT-Festival")]
    MIT_Festival,
    #[serde(rename = "MIT")]
    #[strum(to_string = "MIT")]
    MIT,
    #[serde(rename = "MIT-Modern-Variant")]
    #[strum(to_string = "MIT-Modern-Variant")]
    MIT_Modern_Variant,
    #[serde(rename = "MIT-0")]
    #[strum(to_string = "MIT-0")]
    MIT_0,
    #[serde(rename = "MIT-open-group")]
    #[strum(to_string = "MIT-open-group")]
    MIT_open_group,
    #[serde(rename = "MIT-testregex")]
    #[strum(to_string = "MIT-testregex")]
    MIT_testregex,
    #[serde(rename = "MIT-Wu")]
    #[strum(to_string = "MIT-Wu")]
    MIT_Wu,
    #[serde(rename = "MMIXware")]
    #[strum(to_string = "MMIXware")]
    MMIXware,
    #[serde(rename = "Motosoto")]
    #[strum(to_string = "Motosoto")]
    Motosoto,
    #[serde(rename = "MPL-1.0")]
    #[strum(to_string = "MPL-1.0")]
    MPL_1_0,
    #[serde(rename = "MPL-1.1")]
    #[strum(to_string = "MPL-1.1")]
    MPL_1_1,
    #[serde(rename = "MPL-2.0")]
    #[strum(to_string = "MPL-2.0")]
    MPL_2_0,
    #[serde(rename = "MPL-2.0-no-copyleft-exception")]
    #[strum(to_string = "MPL-2.0-no-copyleft-exception")]
    MPL_2_0_no_copyleft_exception,
    #[serde(rename = "MPEG-SSG")]
    #[strum(to_string = "MPEG-SSG")]
    MPEG_SSG,
    #[serde(rename = "mpi-permissive")]
    #[strum(to_string = "mpi-permissive")]
    mpi_permissive,
    #[serde(rename = "mpich2")]
    #[strum(to_string = "mpich2")]
    mpich2,
    #[serde(rename = "mplus")]
    #[strum(to_string = "mplus")]
    mplus,
    #[serde(rename = "MulanPSL-1.0")]
    #[strum(to_string = "MulanPSL-1.0")]
    MulanPSL_1_0,
    #[serde(rename = "MulanPSL-2.0")]
    #[strum(to_string = "MulanPSL-2.0")]
    MulanPSL_2_0,
    #[serde(rename = "Multics")]
    #[strum(to_string = "Multics")]
    Multics,
    #[serde(rename = "Mup")]
    #[strum(to_string = "Mup")]
    Mup,
    #[serde(rename = "NAIST-2003")]
    #[strum(to_string = "NAIST-2003")]
    NAIST_2003,
    #[serde(rename = "NASA-1.3")]
    #[strum(to_string = "NASA-1.3")]
    NASA_1_3,
    #[serde(rename = "Naumen")]
    #[strum(to_string = "Naumen")]
    Naumen,
    #[serde(rename = "NBPL-1.0")]
    #[strum(to_string = "NBPL-1.0")]
    NBPL_1_0,
    #[serde(rename = "Net-SNMP")]
    #[strum(to_string = "Net-SNMP")]
    Net_SNMP,
    #[serde(rename = "NetCDF")]
    #[strum(to_string = "NetCDF")]
    NetCDF,
    #[serde(rename = "NGPL")]
    #[strum(to_string = "NGPL")]
    NGPL,
    #[serde(rename = "NOSL")]
    #[strum(to_string = "NOSL")]
    NOSL,
    #[serde(rename = "NPL-1.0")]
    #[strum(to_string = "NPL-1.0")]
    NPL_1_0,
    #[serde(rename = "NPL-1.1")]
    #[strum(to_string = "NPL-1.1")]
    NPL_1_1,
    #[serde(rename = "Newsletr")]
    #[strum(to_string = "Newsletr")]
    Newsletr,
    #[serde(rename = "NICTA-1.0")]
    #[strum(to_string = "NICTA-1.0")]
    NICTA_1_0,
    #[serde(rename = "NIST-PD")]
    #[strum(to_string = "NIST-PD")]
    NIST_PD,
    #[serde(rename = "NIST-PD-fallback")]
    #[strum(to_string = "NIST-PD-fallback")]
    NIST_PD_fallback,
    #[serde(rename = "NIST-Software")]
    #[strum(to_string = "NIST-Software")]
    NIST_Software,
    #[serde(rename = "NLPL")]
    #[strum(to_string = "NLPL")]
    NLPL,
    #[serde(rename = "Nokia")]
    #[strum(to_string = "Nokia")]
    Nokia,
    #[serde(rename = "NCGL-UK-2.0")]
    #[strum(to_string = "NCGL-UK-2.0")]
    NCGL_UK_2_0,
    #[serde(rename = "NPOSL-3.0")]
    #[strum(to_string = "NPOSL-3.0")]
    NPOSL_3_0,
    #[serde(rename = "NLOD-1.0")]
    #[strum(to_string = "NLOD-1.0")]
    NLOD_1_0,
    #[serde(rename = "NLOD-2.0")]
    #[strum(to_string = "NLOD-2.0")]
    NLOD_2_0,
    #[serde(rename = "Noweb")]
    #[strum(to_string = "Noweb")]
    Noweb,
    #[serde(rename = "NRL")]
    #[strum(to_string = "NRL")]
    NRL,
    #[serde(rename = "NTP")]
    #[strum(to_string = "NTP")]
    NTP,
    #[serde(rename = "NTP-0")]
    #[strum(to_string = "NTP-0")]
    NTP_0,
    #[serde(rename = "OCLC-2.0")]
    #[strum(to_string = "OCLC-2.0")]
    OCLC_2_0,
    #[serde(rename = "OFFIS")]
    #[strum(to_string = "OFFIS")]
    OFFIS,
    #[serde(rename = "OGC-1.0")]
    #[strum(to_string = "OGC-1.0")]
    OGC_1_0,
    #[serde(rename = "OCCT-PL")]
    #[strum(to_string = "OCCT-PL")]
    OCCT_PL,
    #[serde(rename = "ODC-By-1.0")]
    #[strum(to_string = "ODC-By-1.0")]
    ODC_By_1_0,
    #[serde(rename = "ODbL-1.0")]
    #[strum(to_string = "ODbL-1.0")]
    ODbL_1_0,
    #[serde(rename = "PDDL-1.0")]
    #[strum(to_string = "PDDL-1.0")]
    PDDL_1_0,
    #[serde(rename = "OGL-Canada-2.0")]
    #[strum(to_string = "OGL-Canada-2.0")]
    OGL_Canada_2_0,
    #[serde(rename = "OGL-UK-1.0")]
    #[strum(to_string = "OGL-UK-1.0")]
    OGL_UK_1_0,
    #[serde(rename = "OGL-UK-2.0")]
    #[strum(to_string = "OGL-UK-2.0")]
    OGL_UK_2_0,
    #[serde(rename = "OGL-UK-3.0")]
    #[strum(to_string = "OGL-UK-3.0")]
    OGL_UK_3_0,
    #[serde(rename = "OGTSL")]
    #[strum(to_string = "OGTSL")]
    OGTSL,
    #[serde(rename = "OLDAP-2.2.2")]
    #[strum(to_string = "OLDAP-2.2.2")]
    OLDAP_2_2_2,
    #[serde(rename = "OLDAP-1.1")]
    #[strum(to_string = "OLDAP-1.1")]
    OLDAP_1_1,
    #[serde(rename = "OLDAP-1.2")]
    #[strum(to_string = "OLDAP-1.2")]
    OLDAP_1_2,
    #[serde(rename = "OLDAP-1.3")]
    #[strum(to_string = "OLDAP-1.3")]
    OLDAP_1_3,
    #[serde(rename = "OLDAP-1.4")]
    #[strum(to_string = "OLDAP-1.4")]
    OLDAP_1_4,
    #[serde(rename = "OLDAP-2.0")]
    #[strum(to_string = "OLDAP-2.0")]
    OLDAP_2_0,
    #[serde(rename = "OLDAP-2.0.1")]
    #[strum(to_string = "OLDAP-2.0.1")]
    OLDAP_2_0_1,
    #[serde(rename = "OLDAP-2.1")]
    #[strum(to_string = "OLDAP-2.1")]
    OLDAP_2_1,
    #[serde(rename = "OLDAP-2.2")]
    #[strum(to_string = "OLDAP-2.2")]
    OLDAP_2_2,
    #[serde(rename = "OLDAP-2.2.1")]
    #[strum(to_string = "OLDAP-2.2.1")]
    OLDAP_2_2_1,
    #[serde(rename = "OLDAP-2.3")]
    #[strum(to_string = "OLDAP-2.3")]
    OLDAP_2_3,
    #[serde(rename = "OLDAP-2.4")]
    #[strum(to_string = "OLDAP-2.4")]
    OLDAP_2_4,
    #[serde(rename = "OLDAP-2.5")]
    #[strum(to_string = "OLDAP-2.5")]
    OLDAP_2_5,
    #[serde(rename = "OLDAP-2.6")]
    #[strum(to_string = "OLDAP-2.6")]
    OLDAP_2_6,
    #[serde(rename = "OLDAP-2.7")]
    #[strum(to_string = "OLDAP-2.7")]
    OLDAP_2_7,
    #[serde(rename = "OLDAP-2.8")]
    #[strum(to_string = "OLDAP-2.8")]
    OLDAP_2_8,
    #[serde(rename = "OLFL-1.3")]
    #[strum(to_string = "OLFL-1.3")]
    OLFL_1_3,
    #[serde(rename = "OML")]
    #[strum(to_string = "OML")]
    OML,
    #[serde(rename = "OPL-1.0")]
    #[strum(to_string = "OPL-1.0")]
    OPL_1_0,
    #[serde(rename = "OPUBL-1.0")]
    #[strum(to_string = "OPUBL-1.0")]
    OPUBL_1_0,
    #[serde(rename = "OSL-1.0")]
    #[strum(to_string = "OSL-1.0")]
    OSL_1_0,
    #[serde(rename = "OSL-1.1")]
    #[strum(to_string = "OSL-1.1")]
    OSL_1_1,
    #[serde(rename = "OSL-2.0")]
    #[strum(to_string = "OSL-2.0")]
    OSL_2_0,
    #[serde(rename = "OSL-2.1")]
    #[strum(to_string = "OSL-2.1")]
    OSL_2_1,
    #[serde(rename = "OSL-3.0")]
    #[strum(to_string = "OSL-3.0")]
    OSL_3_0,
    #[serde(rename = "O-UDA-1.0")]
    #[strum(to_string = "O-UDA-1.0")]
    O_UDA_1_0,
    #[serde(rename = "OpenPBS-2.3")]
    #[strum(to_string = "OpenPBS-2.3")]
    OpenPBS_2_3,
    #[serde(rename = "OpenSSL")]
    #[strum(to_string = "OpenSSL")]
    OpenSSL,
    #[serde(rename = "OSET-PL-2.1")]
    #[strum(to_string = "OSET-PL-2.1")]
    OSET_PL_2_1,
    #[serde(rename = "PADL")]
    #[strum(to_string = "PADL")]
    PADL,
    #[serde(rename = "PHP-3.0")]
    #[strum(to_string = "PHP-3.0")]
    PHP_3_0,
    #[serde(rename = "PHP-3.01")]
    #[strum(to_string = "PHP-3.01")]
    PHP_3_01,
    #[serde(rename = "Plexus")]
    #[strum(to_string = "Plexus")]
    Plexus,
    #[serde(rename = "libpng-2.0")]
    #[strum(to_string = "libpng-2.0")]
    libpng_2_0,
    #[serde(rename = "pnmstitch")]
    #[strum(to_string = "pnmstitch")]
    pnmstitch,
    #[serde(rename = "PolyForm-Noncommercial-1.0.0")]
    #[strum(to_string = "PolyForm-Noncommercial-1.0.0")]
    PolyForm_Noncommercial_1_0_0,
    #[serde(rename = "PolyForm-Small-Business-1.0.0")]
    #[strum(to_string = "PolyForm-Small-Business-1.0.0")]
    PolyForm_Small_Business_1_0_0,
    #[serde(rename = "PostgreSQL")]
    #[strum(to_string = "PostgreSQL")]
    PostgreSQL,
    #[serde(rename = "psfrag")]
    #[strum(to_string = "psfrag")]
    psfrag,
    #[serde(rename = "psutils")]
    #[strum(to_string = "psutils")]
    psutils,
    #[serde(rename = "python-ldap")]
    #[strum(to_string = "python-ldap")]
    python_ldap,
    #[serde(rename = "Python-2.0")]
    #[strum(to_string = "Python-2.0")]
    Python_2_0,
    #[serde(rename = "Python-2.0.1")]
    #[strum(to_string = "Python-2.0.1")]
    Python_2_0_1,
    #[serde(rename = "PSF-2.0")]
    #[strum(to_string = "PSF-2.0")]
    PSF_2_0,
    #[serde(rename = "QPL-1.0")]
    #[strum(to_string = "QPL-1.0")]
    QPL_1_0,
    #[serde(rename = "QPL-1.0-INRIA-2004")]
    #[strum(to_string = "QPL-1.0-INRIA-2004")]
    QPL_1_0_INRIA_2004,
    #[serde(rename = "Qhull")]
    #[strum(to_string = "Qhull")]
    Qhull,
    #[serde(rename = "Rdisc")]
    #[strum(to_string = "Rdisc")]
    Rdisc,
    #[serde(rename = "RPSL-1.0")]
    #[strum(to_string = "RPSL-1.0")]
    RPSL_1_0,
    #[serde(rename = "RPL-1.1")]
    #[strum(to_string = "RPL-1.1")]
    RPL_1_1,
    #[serde(rename = "RPL-1.5")]
    #[strum(to_string = "RPL-1.5")]
    RPL_1_5,
    #[serde(rename = "RHeCos-1.1")]
    #[strum(to_string = "RHeCos-1.1")]
    RHeCos_1_1,
    #[serde(rename = "RSCPL")]
    #[strum(to_string = "RSCPL")]
    RSCPL,
    #[serde(rename = "RSA-MD")]
    #[strum(to_string = "RSA-MD")]
    RSA_MD,
    #[serde(rename = "Ruby")]
    #[strum(to_string = "Ruby")]
    Ruby,
    #[serde(rename = "SAX-PD")]
    #[strum(to_string = "SAX-PD")]
    SAX_PD,
    #[serde(rename = "Saxpath")]
    #[strum(to_string = "Saxpath")]
    Saxpath,
    #[serde(rename = "SCEA")]
    #[strum(to_string = "SCEA")]
    SCEA,
    #[serde(rename = "SchemeReport")]
    #[strum(to_string = "SchemeReport")]
    SchemeReport,
    #[serde(rename = "SWL")]
    #[strum(to_string = "SWL")]
    SWL,
    #[serde(rename = "SMPPL")]
    #[strum(to_string = "SMPPL")]
    SMPPL,
    #[serde(rename = "Sendmail")]
    #[strum(to_string = "Sendmail")]
    Sendmail,
    #[serde(rename = "Sendmail-8.23")]
    #[strum(to_string = "Sendmail-8.23")]
    Sendmail_8_23,
    #[serde(rename = "SSPL-1.0")]
    #[strum(to_string = "SSPL-1.0")]
    SSPL_1_0,
    #[serde(rename = "SGI-B-1.0")]
    #[strum(to_string = "SGI-B-1.0")]
    SGI_B_1_0,
    #[serde(rename = "SGI-B-1.1")]
    #[strum(to_string = "SGI-B-1.1")]
    SGI_B_1_1,
    #[serde(rename = "SGI-B-2.0")]
    #[strum(to_string = "SGI-B-2.0")]
    SGI_B_2_0,
    #[serde(rename = "SGI-OpenGL")]
    #[strum(to_string = "SGI-OpenGL")]
    SGI_OpenGL,
    #[serde(rename = "SGP4")]
    #[strum(to_string = "SGP4")]
    SGP4,
    #[serde(rename = "OFL-1.0")]
    #[strum(to_string = "OFL-1.0")]
    OFL_1_0,
    #[serde(rename = "OFL-1.0-no-RFN")]
    #[strum(to_string = "OFL-1.0-no-RFN")]
    OFL_1_0_no_RFN,
    #[serde(rename = "OFL-1.0-RFN")]
    #[strum(to_string = "OFL-1.0-RFN")]
    OFL_1_0_RFN,
    #[serde(rename = "OFL-1.1")]
    #[strum(to_string = "OFL-1.1")]
    OFL_1_1,
    #[serde(rename = "OFL-1.1-no-RFN")]
    #[strum(to_string = "OFL-1.1-no-RFN")]
    OFL_1_1_no_RFN,
    #[serde(rename = "OFL-1.1-RFN")]
    #[strum(to_string = "OFL-1.1-RFN")]
    OFL_1_1_RFN,
    #[serde(rename = "SimPL-2.0")]
    #[strum(to_string = "SimPL-2.0")]
    SimPL_2_0,
    #[serde(rename = "SL")]
    #[strum(to_string = "SL")]
    SL,
    #[serde(rename = "Sleepycat")]
    #[strum(to_string = "Sleepycat")]
    Sleepycat,
    #[serde(rename = "SNIA")]
    #[strum(to_string = "SNIA")]
    SNIA,
    #[serde(rename = "snprintf")]
    #[strum(to_string = "snprintf")]
    snprintf,
    #[serde(rename = "SHL-0.5")]
    #[strum(to_string = "SHL-0.5")]
    SHL_0_5,
    #[serde(rename = "SHL-0.51")]
    #[strum(to_string = "SHL-0.51")]
    SHL_0_51,
    #[serde(rename = "Soundex")]
    #[strum(to_string = "Soundex")]
    Soundex,
    #[serde(rename = "Spencer-86")]
    #[strum(to_string = "Spencer-86")]
    Spencer_86,
    #[serde(rename = "Spencer-94")]
    #[strum(to_string = "Spencer-94")]
    Spencer_94,
    #[serde(rename = "Spencer-99")]
    #[strum(to_string = "Spencer-99")]
    Spencer_99,
    #[serde(rename = "blessing")]
    #[strum(to_string = "blessing")]
    blessing,
    #[serde(rename = "SSH-OpenSSH")]
    #[strum(to_string = "SSH-OpenSSH")]
    SSH_OpenSSH,
    #[serde(rename = "SSH-short")]
    #[strum(to_string = "SSH-short")]
    SSH_short,
    #[serde(rename = "ssh-keyscan")]
    #[strum(to_string = "ssh-keyscan")]
    ssh_keyscan,
    #[serde(rename = "SMLNJ")]
    #[strum(to_string = "SMLNJ")]
    SMLNJ,
    #[serde(rename = "SugarCRM-1.1.3")]
    #[strum(to_string = "SugarCRM-1.1.3")]
    SugarCRM_1_1_3,
    #[serde(rename = "SISSL")]
    #[strum(to_string = "SISSL")]
    SISSL,
    #[serde(rename = "SISSL-1.2")]
    #[strum(to_string = "SISSL-1.2")]
    SISSL_1_2,
    #[serde(rename = "SPL-1.0")]
    #[strum(to_string = "SPL-1.0")]
    SPL_1_0,
    #[serde(rename = "SunPro")]
    #[strum(to_string = "SunPro")]
    SunPro,
    #[serde(rename = "swrule")]
    #[strum(to_string = "swrule")]
    swrule,
    #[serde(rename = "Watcom-1.0")]
    #[strum(to_string = "Watcom-1.0")]
    Watcom_1_0,
    #[serde(rename = "Symlinks")]
    #[strum(to_string = "Symlinks")]
    Symlinks,
    #[serde(rename = "BSD-Systemics")]
    #[strum(to_string = "BSD-Systemics")]
    BSD_Systemics,
    #[serde(rename = "OGDL-Taiwan-1.0")]
    #[strum(to_string = "OGDL-Taiwan-1.0")]
    OGDL_Taiwan_1_0,
    #[serde(rename = "TAPR-OHL-1.0")]
    #[strum(to_string = "TAPR-OHL-1.0")]
    TAPR_OHL_1_0,
    #[serde(rename = "TCL")]
    #[strum(to_string = "TCL")]
    TCL,
    #[serde(rename = "TCP-wrappers")]
    #[strum(to_string = "TCP-wrappers")]
    TCP_wrappers,
    #[serde(rename = "TU-Berlin-1.0")]
    #[strum(to_string = "TU-Berlin-1.0")]
    TU_Berlin_1_0,
    #[serde(rename = "TU-Berlin-2.0")]
    #[strum(to_string = "TU-Berlin-2.0")]
    TU_Berlin_2_0,
    #[serde(rename = "TermReadKey")]
    #[strum(to_string = "TermReadKey")]
    TermReadKey,
    #[serde(rename = "TTWL")]
    #[strum(to_string = "TTWL")]
    TTWL,
    #[serde(rename = "MirOS")]
    #[strum(to_string = "MirOS")]
    MirOS,
    #[serde(rename = "Parity-6.0.0")]
    #[strum(to_string = "Parity-6.0.0")]
    Parity_6_0_0,
    #[serde(rename = "Parity-7.0.0")]
    #[strum(to_string = "Parity-7.0.0")]
    Parity_7_0_0,
    #[serde(rename = "Unlicense")]
    #[strum(to_string = "Unlicense")]
    Unlicense,
    #[serde(rename = "TPL-1.0")]
    #[strum(to_string = "TPL-1.0")]
    TPL_1_0,
    #[serde(rename = "TPDL")]
    #[strum(to_string = "TPDL")]
    TPDL,
    #[serde(rename = "TMate")]
    #[strum(to_string = "TMate")]
    TMate,
    #[serde(rename = "TORQUE-1.1")]
    #[strum(to_string = "TORQUE-1.1")]
    TORQUE_1_1,
    #[serde(rename = "TOSL")]
    #[strum(to_string = "TOSL")]
    TOSL,
    #[serde(rename = "TTYP0")]
    #[strum(to_string = "TTYP0")]
    TTYP0,
    #[serde(rename = "UCAR")]
    #[strum(to_string = "UCAR")]
    UCAR,
    #[serde(rename = "ulem")]
    #[strum(to_string = "ulem")]
    ulem,
    #[serde(rename = "Unicode-DFS-2015")]
    #[strum(to_string = "Unicode-DFS-2015")]
    Unicode_DFS_2015,
    #[serde(rename = "Unicode-DFS-2016")]
    #[strum(to_string = "Unicode-DFS-2016")]
    Unicode_DFS_2016,
    #[serde(rename = "Unicode-TOU")]
    #[strum(to_string = "Unicode-TOU")]
    Unicode_TOU,
    #[serde(rename = "OPL-UK-3.0")]
    #[strum(to_string = "OPL-UK-3.0")]
    OPL_UK_3_0,
    #[serde(rename = "UPL-1.0")]
    #[strum(to_string = "UPL-1.0")]
    UPL_1_0,
    #[serde(rename = "NCSA")]
    #[strum(to_string = "NCSA")]
    NCSA,
    #[serde(rename = "UnixCrypt")]
    #[strum(to_string = "UnixCrypt")]
    UnixCrypt,
    #[serde(rename = "UCL-1.0")]
    #[strum(to_string = "UCL-1.0")]
    UCL_1_0,
    #[serde(rename = "URT-RLE")]
    #[strum(to_string = "URT-RLE")]
    URT_RLE,
    #[serde(rename = "Vim")]
    #[strum(to_string = "Vim")]
    Vim,
    #[serde(rename = "VOSTROM")]
    #[strum(to_string = "VOSTROM")]
    VOSTROM,
    #[serde(rename = "VSL-1.0")]
    #[strum(to_string = "VSL-1.0")]
    VSL_1_0,
    #[serde(rename = "W3C-20150513")]
    #[strum(to_string = "W3C-20150513")]
    W3C_20150513,
    #[serde(rename = "W3C-19980720")]
    #[strum(to_string = "W3C-19980720")]
    W3C_19980720,
    #[serde(rename = "W3C")]
    #[strum(to_string = "W3C")]
    W3C,
    #[serde(rename = "w3m")]
    #[strum(to_string = "w3m")]
    w3m,
    #[serde(rename = "Widget-Workshop")]
    #[strum(to_string = "Widget-Workshop")]
    Widget_Workshop,
    #[serde(rename = "Wsuipa")]
    #[strum(to_string = "Wsuipa")]
    Wsuipa,
    #[serde(rename = "Xnet")]
    #[strum(to_string = "Xnet")]
    Xnet,
    #[serde(rename = "X11")]
    #[strum(to_string = "X11")]
    X11,
    #[serde(rename = "X11-distribute-modifications-variant")]
    #[strum(to_string = "X11-distribute-modifications-variant")]
    X11_distribute_modifications_variant,
    #[serde(rename = "Xdebug-1.03")]
    #[strum(to_string = "Xdebug-1.03")]
    Xdebug_1_03,
    #[serde(rename = "Xerox")]
    #[strum(to_string = "Xerox")]
    Xerox,
    #[serde(rename = "Xfig")]
    #[strum(to_string = "Xfig")]
    Xfig,
    #[serde(rename = "XFree86-1.1")]
    #[strum(to_string = "XFree86-1.1")]
    XFree86_1_1,
    #[serde(rename = "xinetd")]
    #[strum(to_string = "xinetd")]
    xinetd,
    #[serde(rename = "xlock")]
    #[strum(to_string = "xlock")]
    xlock,
    #[serde(rename = "xpp")]
    #[strum(to_string = "xpp")]
    xpp,
    #[serde(rename = "XSkat")]
    #[strum(to_string = "XSkat")]
    XSkat,
    #[serde(rename = "YPL-1.0")]
    #[strum(to_string = "YPL-1.0")]
    YPL_1_0,
    #[serde(rename = "YPL-1.1")]
    #[strum(to_string = "YPL-1.1")]
    YPL_1_1,
    #[serde(rename = "Zed")]
    #[strum(to_string = "Zed")]
    Zed,
    #[serde(rename = "Zeeff")]
    #[strum(to_string = "Zeeff")]
    Zeeff,
    #[serde(rename = "Zend-2.0")]
    #[strum(to_string = "Zend-2.0")]
    Zend_2_0,
    #[serde(rename = "Zimbra-1.3")]
    #[strum(to_string = "Zimbra-1.3")]
    Zimbra_1_3,
    #[serde(rename = "Zimbra-1.4")]
    #[strum(to_string = "Zimbra-1.4")]
    Zimbra_1_4,
    #[serde(rename = "Zlib")]
    #[strum(to_string = "Zlib")]
    Zlib,
    #[serde(rename = "zlib-acknowledgement")]
    #[strum(to_string = "zlib-acknowledgement")]
    zlib_acknowledgement,
    #[serde(rename = "ZPL-1.1")]
    #[strum(to_string = "ZPL-1.1")]
    ZPL_1_1,
    #[serde(rename = "ZPL-2.0")]
    #[strum(to_string = "ZPL-2.0")]
    ZPL_2_0,
    #[serde(rename = "ZPL-2.1")]
    #[strum(to_string = "ZPL-2.1")]
    ZPL_2_1,
}
