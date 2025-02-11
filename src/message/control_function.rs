//! Control function definitions.

use crate::message::data::u7::U7;

/// Custom type for a control function.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ControlFunction(pub U7);

/// Control Functions as defined in the MIDI 1.0 Specification.
///
/// Source: <https://www.midi.org/specifications-old/item/table-3-control-change-messages-data-bytes-2>
#[allow(missing_docs)]
impl ControlFunction {
    pub const BANK_SELECT_0: Self = ControlFunction(U7(0));
    pub const MOD_WHEEL_1: Self = ControlFunction(U7(1));
    pub const BREATH_CONTROLLER_2: Self = ControlFunction(U7(2));
    pub const UNDEFINED_3: Self = ControlFunction(U7(3));
    pub const FOOT_CONTROLLER_4: Self = ControlFunction(U7(4));
    pub const PORTAMENTO_TIME_5: Self = ControlFunction(U7(5));
    pub const DATA_ENTRY_MSB_6: Self = ControlFunction(U7(6));
    pub const CHANNEL_VOLUME_7: Self = ControlFunction(U7(7));
    pub const BALANCE_8: Self = ControlFunction(U7(8));
    pub const UNDEFINED_9: Self = ControlFunction(U7(9));
    pub const PAN_10: Self = ControlFunction(U7(10));
    pub const EXPRESSION_CONTROLLER_11: Self = ControlFunction(U7(11));
    pub const EFFECT_CONTROL_1_12: Self = ControlFunction(U7(12));
    pub const EFFECT_CONTROL_2_13: Self = ControlFunction(U7(13));
    pub const UNDEFINED_14: Self = ControlFunction(U7(14));
    pub const UNDEFINED_15: Self = ControlFunction(U7(15));
    pub const GENERAL_PURPOSE_CONTROLLER_1_16: Self = ControlFunction(U7(16));
    pub const GENERAL_PURPOSE_CONTROLLER_2_17: Self = ControlFunction(U7(17));
    pub const GENERAL_PURPOSE_CONTROLLER_3_18: Self = ControlFunction(U7(18));
    pub const GENERAL_PURPOSE_CONTROLLER_4_19: Self = ControlFunction(U7(19));
    pub const UNDEFINED_20: Self = ControlFunction(U7(20));
    pub const UNDEFINED_21: Self = ControlFunction(U7(21));
    pub const UNDEFINED_22: Self = ControlFunction(U7(22));
    pub const UNDEFINED_23: Self = ControlFunction(U7(23));
    pub const UNDEFINED_24: Self = ControlFunction(U7(24));
    pub const UNDEFINED_25: Self = ControlFunction(U7(25));
    pub const UNDEFINED_26: Self = ControlFunction(U7(26));
    pub const UNDEFINED_27: Self = ControlFunction(U7(27));
    pub const UNDEFINED_28: Self = ControlFunction(U7(28));
    pub const UNDEFINED_29: Self = ControlFunction(U7(29));
    pub const UNDEFINED_30: Self = ControlFunction(U7(30));
    pub const UNDEFINED_31: Self = ControlFunction(U7(31));
    pub const LSB_FOR_BANK_SELECT_32: Self = ControlFunction(U7(32));
    pub const LSB_FOR_MOD_WHEEL_33: Self = ControlFunction(U7(33));
    pub const LSB_FOR_BREATH_CONTROLLER_34: Self = ControlFunction(U7(34));
    pub const LSB_FOR_UNDEFINED_35: Self = ControlFunction(U7(35));
    pub const LSB_FOR_FOOT_CONTROLLER_36: Self = ControlFunction(U7(36));
    pub const LSB_FOR_PORTAMENTO_TIME_37: Self = ControlFunction(U7(37));
    pub const LSB_FOR_DATA_ENTRY_MSB_38: Self = ControlFunction(U7(38));
    pub const LSB_FOR_CHANNEL_VOLUME_39: Self = ControlFunction(U7(39));
    pub const LSB_FOR_BALANCE_40: Self = ControlFunction(U7(40));
    pub const LSB_FOR_UNDEFINED_41: Self = ControlFunction(U7(41));
    pub const LSB_FOR_PAN_42: Self = ControlFunction(U7(42));
    pub const LSB_FOR_EXPRESSION_CONTROLLER_43: Self = ControlFunction(U7(43));
    pub const LSB_FOR_EFFECT_CONTROL_1_44: Self = ControlFunction(U7(44));
    pub const LSB_FOR_EFFECT_CONTROL_2_45: Self = ControlFunction(U7(45));
    pub const LSB_FOR_UNDEFINED_14_46: Self = ControlFunction(U7(46));
    pub const LSB_FOR_UNDEFINED_15_47: Self = ControlFunction(U7(47));
    pub const LSB_FOR_GENERAL_PURPOSE_CONTROLLER_1_48: Self = ControlFunction(U7(48));
    pub const LSB_FOR_GENERAL_PURPOSE_CONTROLLER_2_49: Self = ControlFunction(U7(49));
    pub const LSB_FOR_GENERAL_PURPOSE_CONTROLLER_3_50: Self = ControlFunction(U7(50));
    pub const LSB_FOR_GENERAL_PURPOSE_CONTROLLER_4_51: Self = ControlFunction(U7(51));
    pub const LSB_FOR_UNDEFINED_20_52: Self = ControlFunction(U7(52));
    pub const LSB_FOR_UNDEFINED_21_53: Self = ControlFunction(U7(53));
    pub const LSB_FOR_UNDEFINED_22_54: Self = ControlFunction(U7(54));
    pub const LSB_FOR_UNDEFINED_23_55: Self = ControlFunction(U7(55));
    pub const LSB_FOR_UNDEFINED_24_56: Self = ControlFunction(U7(56));
    pub const LSB_FOR_UNDEFINED_25_57: Self = ControlFunction(U7(57));
    pub const LSB_FOR_UNDEFINED_26_58: Self = ControlFunction(U7(58));
    pub const LSB_FOR_UNDEFINED_27_59: Self = ControlFunction(U7(59));
    pub const LSB_FOR_UNDEFINED_28_60: Self = ControlFunction(U7(60));
    pub const LSB_FOR_UNDEFINED_29_61: Self = ControlFunction(U7(61));
    pub const LSB_FOR_UNDEFINED_30_62: Self = ControlFunction(U7(62));
    pub const LSB_FOR_UNDEFINED_31_63: Self = ControlFunction(U7(63));
    pub const DAMPER_PEDAL_ON_OFF_64: Self = ControlFunction(U7(64));
    pub const PORTAMENTO_ON_OFF_65: Self = ControlFunction(U7(65));
    pub const SOSTENUTO_ON_OFF_66: Self = ControlFunction(U7(66));
    pub const SOFT_PEDAL_ON_OFF_67: Self = ControlFunction(U7(67));
    pub const LEGATO_FOOTSWITCH_68: Self = ControlFunction(U7(68));
    pub const HOLD_2_69: Self = ControlFunction(U7(69));
    pub const SOUND_CONTROLLER_1_70: Self = ControlFunction(U7(70));
    pub const SOUND_CONTROLLER_2_71: Self = ControlFunction(U7(71));
    pub const SOUND_CONTROLLER_3_72: Self = ControlFunction(U7(72));
    pub const SOUND_CONTROLLER_4_73: Self = ControlFunction(U7(73));
    pub const SOUND_CONTROLLER_5_74: Self = ControlFunction(U7(74));
    pub const SOUND_CONTROLLER_6_75: Self = ControlFunction(U7(75));
    pub const SOUND_CONTROLLER_7_76: Self = ControlFunction(U7(76));
    pub const SOUND_CONTROLLER_8_77: Self = ControlFunction(U7(77));
    pub const SOUND_CONTROLLER_9_78: Self = ControlFunction(U7(78));
    pub const SOUND_CONTROLLER_10_79: Self = ControlFunction(U7(79));
    pub const GENERAL_PURPOSE_CONTROLLER_5_80: Self = ControlFunction(U7(80));
    pub const GENERAL_PURPOSE_CONTROLLER_6_81: Self = ControlFunction(U7(81));
    pub const GENERAL_PURPOSE_CONTROLLER_7_82: Self = ControlFunction(U7(82));
    pub const GENERAL_PURPOSE_CONTROLLER_8_83: Self = ControlFunction(U7(83));
    pub const PORTAMENTO_CONTROL_84: Self = ControlFunction(U7(84));
    pub const UNDEFINED_85: Self = ControlFunction(U7(85));
    pub const UNDEFINED_86: Self = ControlFunction(U7(86));
    pub const UNDEFINED_87: Self = ControlFunction(U7(87));
    pub const HIGH_RESOLUTION_VELOCITY_PREFIX_88: Self = ControlFunction(U7(88));
    pub const UNDEFINED_89: Self = ControlFunction(U7(89));
    pub const UNDEFINED_90: Self = ControlFunction(U7(90));
    pub const EFFECTS_1_DEPTH_91: Self = ControlFunction(U7(91));
    pub const EFFECTS_2_DEPTH_92: Self = ControlFunction(U7(92));
    pub const EFFECTS_3_DEPTH_93: Self = ControlFunction(U7(93));
    pub const EFFECTS_4_DEPTH_94: Self = ControlFunction(U7(94));
    pub const EFFECTS_5_DEPTH_95: Self = ControlFunction(U7(95));
    pub const DATA_INCREMENT_96: Self = ControlFunction(U7(96));
    pub const DATA_DECREMENT_97: Self = ControlFunction(U7(97));
    pub const NPRN_LSB_98: Self = ControlFunction(U7(98));
    pub const NPRN_MSB_99: Self = ControlFunction(U7(99));
    pub const RPN_LSB_100: Self = ControlFunction(U7(100));
    pub const UNDEFINED_101: Self = ControlFunction(U7(101));
    pub const UNDEFINED_102: Self = ControlFunction(U7(102));
    pub const UNDEFINED_103: Self = ControlFunction(U7(103));
    pub const UNDEFINED_104: Self = ControlFunction(U7(104));
    pub const UNDEFINED_105: Self = ControlFunction(U7(105));
    pub const UNDEFINED_106: Self = ControlFunction(U7(106));
    pub const UNDEFINED_107: Self = ControlFunction(U7(107));
    pub const UNDEFINED_108: Self = ControlFunction(U7(108));
    pub const UNDEFINED_109: Self = ControlFunction(U7(109));
    pub const UNDEFINED_110: Self = ControlFunction(U7(110));
    pub const UNDEFINED_111: Self = ControlFunction(U7(111));
    pub const UNDEFINED_112: Self = ControlFunction(U7(112));
    pub const UNDEFINED_113: Self = ControlFunction(U7(113));
    pub const UNDEFINED_114: Self = ControlFunction(U7(114));
    pub const UNDEFINED_115: Self = ControlFunction(U7(115));
    pub const UNDEFINED_116: Self = ControlFunction(U7(116));
    pub const UNDEFINED_117: Self = ControlFunction(U7(117));
    pub const UNDEFINED_118: Self = ControlFunction(U7(118));
    pub const UNDEFINED_119: Self = ControlFunction(U7(119));
    pub const ALL_SOUND_OFF_120: Self = ControlFunction(U7(120));
    pub const RESET_ALL_CONTROLLERS_121: Self = ControlFunction(U7(121));
    pub const LOCAL_CONTROL_OFF_122: Self = ControlFunction(U7(122));
    pub const ALL_NOTES_OFF_123: Self = ControlFunction(U7(123));
    pub const OMNI_MODE_OFF_124: Self = ControlFunction(U7(124));
    pub const OMNI_MODE_ON_125: Self = ControlFunction(U7(125));
    pub const MONO_MODE_ON_126: Self = ControlFunction(U7(126));
    pub const POLY_MODE_ON_127: Self = ControlFunction(U7(127));
}
