use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, newline, not_line_ending, space1};
use nom::multi::{many0, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Needed};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    pub t: String,
    pub tags: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Zjkg {
    pub ver: u8,
    pub mid: String,
    pub nm: String,
    pub images: Vec<Image>,
}

// 解析服务器时间
fn parse_server_time(input: &str) -> IResult<&str, &str> {
    let mut parser = tuple((
        digit1,
        tag("-"),
        digit1,
        tag("-"),
        digit1,
        space1,
        digit1,
        tag(":"),
        digit1,
        tag(":"),
        digit1,
        tag("."),
        digit1,
    ));
    let (input, (year, _, month, _, day, _, hour, _, minute, _, seconds, _, micro_sec)) =
        parser(input)?;
    Ok((input, year))
}

fn parse_topic_name(input: &str) -> IResult<&str, Vec<&str>> {
    let mut parser = tuple((tag("["), separated_list1(tag("/"), alphanumeric1), tag("]")));
    let (input, (_, topics, _)) = parser(input)?;
    Ok((input, topics))
}

fn parse_json_str(input: &str) -> IResult<&str, Zjkg> {
    let mut parser = tuple((tag("D:"), not_line_ending));
    let (input, (_, json)) = parser(input)?;

    let parsed = serde_json::from_str::<Zjkg>(json);
    if let Ok(parsed) = parsed {
        Ok((input, parsed))
    } else {
        Err(nom::Err::Incomplete(Needed::Unknown))
    }
}

pub fn parse_zjkg_log(input: &str) -> IResult<&str, Zjkg> {
    let mut parser = tuple((
        parse_server_time,
        space1,
        parse_topic_name,
        space1,
        parse_json_str,
        many0(newline),
    ));
    let (input, (ts, _, topic_name, _, json_str, _)) = parser(input)?;
    Ok((input, json_str))
}

#[test]
fn test_server_time() {
    let input = "2024-05-05 00:00:21.525";
    assert_eq!(parse_server_time(input), Ok(("", "2024")));
}

#[test]
fn test_topic_name() {
    let input = "[zjkg]";
    assert_eq!(parse_topic_name(input), Ok(("", vec!["zjkg"])));
}

#[test]
fn test_json_str() {
    let input = r#"2024-05-05 00:00:21.525  [zjkg]  D:{"ver":211,"mid":"pack2","nm":"pack2","images":[{"t":"2024-05-05 00:00:19.009","tags":{"BMS_pack_2_ele_MaxDisChgPwr":0.0,"BMS_pack_2_ele_MaxChgU":0.0,"BMS_pack_2_ele_MaxDisChgU":0.0,"BMS_pack_2_ele_MaxChgI":0.0,"BMS_pack_2_ele_MaxDisChgI":0.0,"BMS_pack_2_ele_u":672.4,"BMS_pack_2_ele_i":0.0,"BMS_pack_2_ele_temp":33.0,"BMS_pack_2_ele_soc":47.0,"BMS_pack_2_ele_soh":100.0,"BMS_pack_2_ele_InsulRes":12135.0,"BMS_pack_2_ele_TolChgVol":7437.9,"BMS_pack_2_ele_TolDischgVol":6793.1,"BMS_pack_2_ele_SglTolChgVol":0.0,"BMS_pack_2_ele_SglTolDisChgVol":2.8,"BMS_pack_2_ele_CapChg":51.8,"BMS_pack_2_ele_CapDisChg":46.0,"BMS_pack_2_ele_MaxChgPwr":0.0,"BMS_pack_IoStatus":1,"BMS_pack_2_sts_sts_2":8,"BMS_pack_2_sts_input_1":0.0,"BMS_pack_2_sts_input_2":0.0,"BMS_pack_2_sts_input_3":0.0,"BMS_pack_2_sts_input_4":0.0,"BMS_pack_2_sts_input_5":0.0,"BMS_pack_2_sts_input_6":0.0,"BMS_pack_2_sts_input_7":0.0,"BMS_pack_2_sts_input_8":0.0,"BMS_cell_2_u_1":3.306,"BMS_cell_2_u_2":3.321,"BMS_cell_2_u_3":3.329,"BMS_cell_2_u_4":3.329,"BMS_cell_2_u_5":3.304,"BMS_cell_2_u_6":3.327,"BMS_cell_2_u_7":3.314,"BMS_cell_2_u_8":3.312,"BMS_cell_2_u_9":3.329,"BMS_cell_2_u_10":3.319,"BMS_cell_2_u_11":3.317,"BMS_cell_2_u_12":3.329,"BMS_cell_2_u_13":3.308,"BMS_cell_2_u_14":3.314,"BMS_cell_2_u_15":3.309,"BMS_cell_2_u_16":3.303,"BMS_cell_2_u_17":3.323,"BMS_cell_2_u_18":3.304,"BMS_cell_2_u_19":3.317,"BMS_cell_2_u_20":3.311,"BMS_cell_2_u_21":3.313,"BMS_cell_2_u_22":3.314,"BMS_cell_2_u_23":3.315,"BMS_cell_2_u_24":3.314,"BMS_cell_2_u_25":3.309,"BMS_cell_2_u_26":3.306,"BMS_cell_2_u_27":3.31,"BMS_cell_2_u_28":3.308,"BMS_cell_2_u_29":3.305,"BMS_cell_2_u_30":3.31,"BMS_cell_2_u_31":3.308,"BMS_cell_2_u_32":3.316,"BMS_cell_2_u_33":3.317,"BMS_cell_2_u_34":3.313,"BMS_cell_2_u_35":3.311,"BMS_cell_2_u_36":3.311,"BMS_cell_2_u_37":3.303,"BMS_cell_2_u_38":3.318,"BMS_cell_2_u_39":3.302,"BMS_cell_2_u_40":3.311,"BMS_cell_2_u_41":3.32,"BMS_cell_2_u_42":3.303,"BMS_cell_2_u_43":3.31,"BMS_cell_2_u_44":3.303,"BMS_cell_2_u_45":3.303,"BMS_cell_2_u_46":3.305,"BMS_cell_2_u_47":3.306,"BMS_cell_2_u_48":3.298,"BMS_cell_2_u_49":3.314,"BMS_cell_2_u_50":3.308,"BMS_cell_2_u_51":3.306,"BMS_cell_2_u_52":3.309,"BMS_cell_2_u_53":3.311,"BMS_cell_2_u_54":3.317,"BMS_cell_2_u_55":3.312,"BMS_cell_2_u_56":3.306,"BMS_cell_2_u_57":3.317,"BMS_cell_2_u_58":3.308,"BMS_cell_2_u_59":3.309,"BMS_cell_2_u_60":3.311,"BMS_cell_2_u_61":3.3,"BMS_cell_2_u_62":3.3,"BMS_cell_2_u_63":3.3,"BMS_cell_2_u_64":3.297,"BMS_cell_2_u_65":3.3,"BMS_cell_2_u_66":3.302,"BMS_cell_2_u_67":3.3,"BMS_cell_2_u_68":3.299,"BMS_cell_2_u_69":3.298,"BMS_cell_2_u_70":3.3,"BMS_cell_2_u_71":3.299,"BMS_cell_2_u_72":3.302,"BMS_cell_2_u_73":3.303,"BMS_cell_2_u_74":3.303,"BMS_cell_2_u_75":3.3,"BMS_cell_2_u_76":3.303,"BMS_cell_2_u_77":3.302,"BMS_cell_2_u_78":3.302,"BMS_cell_2_u_79":3.304,"BMS_cell_2_u_80":3.303,"BMS_cell_2_u_81":3.306,"BMS_cell_2_u_82":3.302,"BMS_cell_2_u_83":3.304,"BMS_cell_2_u_84":3.3,"BMS_cell_2_u_85":3.303,"BMS_cell_2_u_86":3.305,"BMS_cell_2_u_87":3.314,"BMS_cell_2_u_88":3.328,"BMS_cell_2_u_89":3.316,"BMS_cell_2_u_90":3.303,"BMS_cell_2_u_91":3.313,"BMS_cell_2_u_92":3.318,"BMS_cell_2_u_93":3.302,"BMS_cell_2_u_94":3.317,"BMS_cell_2_u_95":3.309,"BMS_cell_2_u_96":3.315,"BMS_cell_2_u_97":3.318,"BMS_cell_2_u_98":3.308,"BMS_cell_2_u_99":3.317,"BMS_cell_2_u_100":3.326,"BMS_cell_2_u_101":3.307,"BMS_cell_2_u_102":3.309,"BMS_cell_2_u_103":3.314,"BMS_cell_2_u_104":3.305,"BMS_cell_2_u_105":3.305,"BMS_cell_2_u_106":3.306,"BMS_cell_2_u_107":3.314,"BMS_cell_2_u_108":3.323,"BMS_cell_2_u_109":3.311,"BMS_cell_2_u_110":3.329,"BMS_cell_2_u_111":3.329,"BMS_cell_2_u_112":3.32,"BMS_cell_2_u_113":3.329,"BMS_cell_2_u_114":3.329,"BMS_cell_2_u_115":3.314,"BMS_cell_2_u_116":3.309,"BMS_cell_2_u_117":3.324,"BMS_cell_2_u_118":3.309,"BMS_cell_2_u_119":3.318,"BMS_cell_2_u_120":3.324,"BMS_cell_2_u_121":3.305,"BMS_cell_2_u_122":3.308,"BMS_cell_2_u_123":3.3,"BMS_cell_2_u_124":3.309,"BMS_cell_2_u_125":3.31,"BMS_cell_2_u_126":3.302,"BMS_cell_2_u_127":3.305,"BMS_cell_2_u_128":3.302,"BMS_cell_2_u_129":3.305,"BMS_cell_2_u_130":3.3,"BMS_cell_2_u_131":3.302,"BMS_cell_2_u_132":3.312,"BMS_cell_2_u_133":3.313,"BMS_cell_2_u_134":3.306,"BMS_cell_2_u_135":3.312,"BMS_cell_2_u_136":3.31,"BMS_cell_2_u_137":3.301,"BMS_cell_2_u_138":3.299,"BMS_cell_2_u_139":3.315,"BMS_cell_2_u_140":3.315,"BMS_cell_2_u_141":3.299,"BMS_cell_2_u_142":3.313,"BMS_cell_2_u_143":3.302,"BMS_cell_2_u_144":3.315,"BMS_cell_2_u_145":3.303,"BMS_cell_2_u_146":3.308,"BMS_cell_2_u_147":3.308,"BMS_cell_2_u_148":3.302,"BMS_cell_2_u_149":3.309,"BMS_cell_2_u_150":3.314,"BMS_cell_2_u_151":3.323,"BMS_cell_2_u_152":3.303,"BMS_cell_2_u_153":3.323,"BMS_cell_2_u_154":3.299,"BMS_cell_2_u_155":3.318,"BMS_cell_2_u_156":3.305,"BMS_cell_2_u_157":3.326,"BMS_cell_2_u_158":3.317,"BMS_cell_2_u_159":3.307,"BMS_cell_2_u_160":3.311,"BMS_cell_2_u_161":3.308,"BMS_cell_2_u_162":3.312,"BMS_cell_2_u_163":3.311,"BMS_cell_2_u_164":3.308,"BMS_cell_2_u_165":3.317,"BMS_cell_2_u_166":3.306,"BMS_cell_2_u_167":3.309,"BMS_cell_2_u_168":3.314,"BMS_cell_2_u_169":3.303,"BMS_cell_2_u_170":3.317,"BMS_cell_2_u_171":3.314,"BMS_cell_2_u_172":3.305,"BMS_cell_2_u_173":3.318,"BMS_cell_2_u_174":3.308,"BMS_cell_2_u_175":3.319,"BMS_cell_2_u_176":3.309,"BMS_cell_2_u_177":3.314,"BMS_cell_2_u_178":3.309,"BMS_cell_2_u_179":3.312,"BMS_cell_2_u_180":3.324,"BMS_cell_2_u_181":3.308,"BMS_cell_2_u_182":3.308,"BMS_cell_2_u_183":3.303,"BMS_cell_2_u_184":3.312,"BMS_cell_2_u_185":3.308,"BMS_cell_2_u_186":3.329,"BMS_cell_2_u_187":3.329,"BMS_cell_2_u_188":3.329,"BMS_cell_2_u_189":3.327,"BMS_cell_2_u_190":3.329,"BMS_cell_2_u_191":3.327,"BMS_cell_2_u_192":3.329,"BMS_cell_2_u_193":3.306,"BMS_cell_2_u_194":3.306,"BMS_cell_2_u_195":3.309,"BMS_cell_2_u_196":3.311,"BMS_cell_2_u_197":3.306,"BMS_cell_2_u_198":3.317,"BMS_cell_2_u_199":3.306,"BMS_cell_2_u_200":3.305,"BMS_cell_2_u_201":3.305,"BMS_cell_2_u_202":3.304,"BMS_cell_2_u_203":3.305,"BMS_cell_2_u_204":3.311,"BMS_cell_IoStatus":1,"BMS_cell_2_temp_1":21.0,"BMS_cell_2_temp_2":21.0,"BMS_cell_2_temp_3":21.0,"BMS_cell_2_temp_4":21.0,"BMS_cell_2_temp_5":21.0,"BMS_cell_2_temp_6":21.0,"BMS_cell_2_temp_7":20.0,"BMS_cell_2_temp_8":20.0,"BMS_cell_2_temp_9":21.0,"BMS_cell_2_temp_10":21.0,"BMS_cell_2_temp_11":21.0,"BMS_cell_2_temp_12":21.0,"BMS_cell_2_temp_13":21.0,"BMS_cell_2_temp_14":21.0,"BMS_cell_2_temp_15":21.0,"BMS_cell_2_temp_16":21.0,"BMS_cell_2_temp_17":21.0,"BMS_cell_2_temp_18":21.0,"BMS_cell_2_temp_19":21.0,"BMS_cell_2_temp_20":20.0,"BMS_cell_2_temp_21":21.0,"BMS_cell_2_temp_22":21.0,"BMS_cell_2_temp_23":21.0,"BMS_cell_2_temp_24":21.0,"BMS_cell_2_temp_25":20.0,"BMS_cell_2_temp_26":20.0,"BMS_cell_2_temp_27":21.0,"BMS_cell_2_temp_28":21.0,"BMS_cell_2_temp_29":20.0,"BMS_cell_2_temp_30":21.0,"BMS_cell_2_temp_31":20.0,"BMS_cell_2_temp_32":20.0,"BMS_cell_2_temp_33":20.0,"BMS_cell_2_temp_34":20.0,"BMS_cell_2_temp_35":21.0,"BMS_cell_2_temp_36":20.0,"BMS_cell_2_temp_37":21.0,"BMS_cell_2_temp_38":21.0,"BMS_cell_2_temp_39":21.0,"BMS_cell_2_temp_40":20.0,"BMS_cell_2_temp_41":20.0,"BMS_cell_2_temp_42":20.0,"BMS_cell_2_temp_43":21.0,"BMS_cell_2_temp_44":21.0,"BMS_cell_2_temp_45":21.0,"BMS_cell_2_temp_46":21.0,"BMS_cell_2_temp_47":21.0,"BMS_cell_2_temp_48":20.0,"BMS_cell_2_temp_49":21.0,"BMS_cell_2_temp_50":20.0,"BMS_cell_2_temp_51":20.0,"BMS_cell_2_temp_52":21.0,"BMS_cell_2_temp_53":21.0,"BMS_cell_2_temp_54":21.0,"BMS_cell_2_temp_55":20.0,"BMS_cell_2_temp_56":20.0,"BMS_cell_2_temp_57":21.0,"BMS_cell_2_temp_58":20.0,"BMS_cell_2_temp_59":20.0,"BMS_cell_2_temp_60":20.0,"BMS_cell_2_temp_61":20.0,"BMS_cell_2_temp_62":20.0,"BMS_cell_2_temp_63":21.0,"BMS_cell_2_temp_64":20.0,"BMS_cell_2_temp_65":21.0,"BMS_cell_2_temp_66":20.0,"BMS_cell_2_temp_67":21.0,"BMS_cell_2_temp_68":20.0,"BMS_cell_2_temp_69":21.0,"BMS_cell_2_temp_70":20.0,"BMS_cell_2_temp_71":20.0,"BMS_cell_2_temp_72":21.0,"BMS_cell_2_temp_73":21.0,"BMS_cell_2_temp_74":21.0,"BMS_cell_2_temp_75":21.0,"BMS_cell_2_temp_76":21.0,"BMS_cell_2_temp_77":21.0,"BMS_cell_2_temp_78":21.0,"BMS_cell_2_temp_79":21.0,"BMS_cell_2_temp_80":21.0,"BMS_cell_2_temp_81":21.0,"BMS_cell_2_temp_82":21.0,"BMS_cell_2_temp_83":21.0,"BMS_cell_2_temp_84":21.0,"BMS_cell_2_temp_85":20.0,"BMS_cell_2_temp_86":21.0,"BMS_cell_2_temp_87":21.0,"BMS_cell_2_temp_88":21.0,"BMS_cell_2_temp_89":20.0,"BMS_cell_2_temp_90":20.0,"BMS_cell_2_temp_91":20.0,"BMS_cell_2_temp_92":21.0,"BMS_cell_2_temp_93":21.0,"BMS_cell_2_temp_94":21.0,"BMS_cell_2_temp_95":20.0,"BMS_cell_2_temp_96":20.0,"BMS_cell_2_temp_97":20.0,"BMS_cell_2_temp_98":21.0,"BMS_cell_2_temp_99":20.0,"BMS_cell_2_temp_100":20.0,"BMS_cell_2_temp_101":20.0,"BMS_cell_2_temp_102":20.0,"BMS_pack_2_alarm_300":0.0,"BMS_pack_2_alarm_301":0.0,"BMS_pack_2_alarm_302":0.0,"BMS_pack_2_alarm_303":0.0,"BMS_pack_2_alarm_304":0.0,"BMS_pack_2_alarm_305":0.0,"BMS_pack_2_alarm_306":0.0,"BMS_pack_2_alarm_307":0.0,"BMS_pack_2_alarm_308":0.0,"BMS_pack_2_alarm_309":0.0,"BMS_pack_2_alarm_310":0.0,"BMS_pack_2_alarm_311":0.0,"BMS_pack_2_alarm_312":0.0,"BMS_pack_2_alarm_313":0.0,"BMS_pack_2_alarm_314":0.0,"BMS_pack_2_alarm_315":0.0,"BMS_pack_2_alarm_316":0.0,"BMS_pack_2_alarm_317":0.0,"BMS_pack_2_alarm_318":0.0,"BMS_pack_2_alarm_319":0.0,"BMS_pack_2_alarm_320":0.0,"BMS_pack_2_alarm_321":0.0,"BMS_pack_2_alarm_322":0.0,"BMS_pack_2_alarm_323":0.0,"BMS_pack_2_alarm_324":0.0,"BMS_pack_2_alarm_325":0.0,"BMS_pack_2_alarm_326":0.0,"BMS_pack_2_alarm_327":0.0,"BMS_pack_2_alarm_328":0.0,"BMS_pack_2_alarm_329":0.0,"BMS_pack_2_alarm_330":0.0,"BMS_pack_2_alarm_331":0.0,"BMS_pack_2_alarm_332":0.0,"BMS_pack_2_alarm_333":0.0,"BMS_pack_2_alarm_334":0.0,"BMS_pack_2_alarm_335":0.0,"BMS_pack_2_alarm_336":0.0,"BMS_pack_2_alarm_337":0.0,"BMS_pack_2_alarm_338":0.0,"BMS_pack_2_alarm_339":0.0,"BMS_pack_2_alarm_340":0.0,"BMS_pack_2_alarm_341":0.0,"BMS_pack_2_alarm_342":0.0,"BMS_pack_2_alarm_343":0.0,"BMS_pack_2_alarm_344":0.0,"BMS_pack_2_alarm_345":0.0,"BMS_pack_2_alarm_346":0.0,"BMS_pack_2_alarm_347":0.0,"BMS_pack_2_alarm_348":0.0,"BMS_pack_2_alarm_349":0.0,"BMS_pack_2_alarm_350":0.0,"BMS_pack_2_alarm_351":0.0,"BMS_pack_2_alarm_352":0.0,"BMS_pack_2_alarm_353":0.0,"BMS_pack_2_alarm_354":0.0,"BMS_pack_2_alarm_355":0.0,"BMS_pack_2_alarm_356":0.0,"BMS_pack_2_alarm_357":0.0,"BMS_pack_2_alarm_358":0.0,"BMS_pack_2_alarm_359":0.0,"BMS_pack_2_alarm_360":0.0,"BMS_pack_2_alarm_361":0.0,"BMS_pack_2_alarm_362":0.0,"BMS_pack_2_alarm_363":0.0,"BMS_pack_2_alarm_364":0.0,"BMS_pack_2_alarm_365":0.0,"BMS_pack_2_alarm_366":0.0,"BMS_pack_2_alarm_367":0.0,"BMS_pack_2_alarm_368":0.0,"BMS_pack_2_alarm_369":0.0,"BMS_pack_2_alarm_370":0.0,"BMS_pack_2_alarm_371":0.0,"BMS_pack_2_alarm_372":0.0,"BMS_pack_2_alarm_373":0.0,"BMS_pack_2_alarm_374":0.0,"BMS_pack_2_alarm_375":0.0,"BMS_pack_2_alarm_376":0.0,"BMS_pack_2_alarm_377":0.0,"BMS_pack_2_alarm_378":0.0,"BMS_pack_2_alarm_379":0.0,"BMS_pack_2_alarm_380":0.0,"BMS_pack_2_alarm_381":0.0,"BMS_pack_2_alarm_382":0.0,"BMS_pack_2_alarm_383":0.0,"BMS_pack_2_alarm_384":0.0,"BMS_pack_2_alarm_385":0.0,"BMS_pack_2_alarm_386":0.0,"BMS_pack_2_alarm_387":0.0}}]}"#;
    if let Ok(res) = parse_zjkg_log(input) {
        println!("{:?}", res.1);
    } else {
        println!("{}", input);
    }
}
