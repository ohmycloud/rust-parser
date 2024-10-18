mod with_nom;

use with_nom::parse_log;

fn main() {
    let input = r#"2024-05-05 00:00:03.294  [byts/DATA/CN18b08cf88f1]  D:{"data":{"3880":"50","3881":"55","3640":"7","3882":"60","3638":"7","3639":"7","3630":"7","3872":"155","3631":"7","3873":"180","3632":"7","3874":"200","3633":"6","3875":"10","3634":"8","3876":"0","3635":"6","3877":"0","3636":"7","3878":"500","3637":"8","3879":"100","3890":"3.7","3891":"0.2","3650":"10","3892":"3","3651":"10","3893":"2.8","3406":"3.141","3649":"10","3405":"3.14","3404":"3.144","3403":"3.162","3402":"3.152","3401":"3.157","3400":"3.152","3641":"6","3883":"5","801":"1","3642":"7","3884":"5","802":"0","3643":"7","3885":"0","803":"0","3644":"11","3886":"-5","804":"0","3645":"11","3887":"5","805":"0","3409":"3.141","3646":"7","3888":"3.55","806":"0","3408":"3.146","3647":"11","3889":"3.6","807":"0","3407":"3.137","3648":"11","808":"0","809":"0","3660":"10","3661":"10","3662":"9","1000":"0","3420":"3.123","3417":"3.11","3416":"3.112","3415":"3.103","3414":"3.105","3413":"3.121","3412":"3.145","3411":"3.149","810":"0","3410":"3.145","811":"0","3652":"10","3894":"2.5","812":"0","3653":"10","3895":"0.2","813":"0","3654":"10","3896":"0.3","814":"0","3655":"10","3897":"0.5","815":"0","3656":"11","3898":"1","3657":"10","3899":"0.2","3419":"3.115","3658":"10","3418":"3.11","3659":"10","3670":"7","3671":"9","3672":"11","3673":"8","1011":"0","3431":"3.124","1010":"0","3430":"3.123","1008":"0","3428":"3.127","1007":"0","3427":"3.121","1006":"0","3426":"3.116","1005":"0","3425":"3.123","1004":"0","3424":"3.12","1003":"0","3423":"3.12","1002":"0","3422":"3.12","1001":"0","3421":"3.122","3663":"10","3664":"8","3665":"11","3666":"8","3667":"9","3668":"11","3669":"9","1009":"0","3429":"3.124","3680":"10","3681":"11","3682":"10","3683":"8","3684":"8","1022":"0","3200":"49.9","3442":"29","1021":"0","3441":"29","1020":"0","3440":"29","1019":"0","3439":"29","1018":"0","3438":"29","1017":"0","3437":"29","1016":"0","3436":"29","1015":"0","3435":"3.12","1014":"0","3434":"3.124","831":"1","1013":"0","3433":"3.123","832":"0","1012":"0","3432":"3.122","833":"0","3674":"11","834":"0","3675":"11","835":"0","3676":"11","836":"0","3677":"8","837":"0","3678":"11","838":"0","3679":"11","839":"0","3690":"9","3691":"10","3692":"9","3693":"7","3694":"7","3695":"9","1033":"0","3211":"2","3453":"29","1032":"0","3210":"0.087","3452":"29","1031":"0","3451":"28","1030":"0","3450":"29","3208":"180","1029":"0","3207":"50","3449":"30","1028":"0","3206":"100","3448":"29","1027":"0","3205":"167","3447":"29","840":"0","1026":"0","3204":"0","3446":"29","841":"0","1025":"0","3203":"38868.2","3445":"29","842":"0","1024":"0","3202":"40891.6","3444":"29","843":"0","1023":"0","3201":"14","3443":"29","844":"0","3685":"8","845":"0","3686":"8","3687":"7","3688":"8","3689":"8","3209":"28","1044":"0","3222":"3.149","3464":"29","1043":"0","3221":"3.149","3463":"29","1042":"1","3220":"3.156","3462":"29","3461":"29","3460":"29","3219":"3.14","3218":"3.119","3217":"3.137","3459":"29","3216":"3.138","3458":"29","3215":"3.132","3457":"29","3214":"3.131","3456":"29","3213":"3.142","3455":"30","1034":"0","3212":"3.142","3454":"29","3696":"7","3697":"7","3698":"7","3699":"7","3233":"3.177","3475":"28","3232":"3.177","3474":"29","1053":"0","3231":"3.177","3473":"29","1052":"0","3230":"3.177","3472":"29","1051":"0","3471":"29","1050":"0","3470":"29","3229":"3.176","3228":"3.18","861":"1","1049":"0","3227":"3.156","3469":"29","1048":"0","3226":"3.147","3468":"29","863":"1","1047":"0","3225":"3.143","3467":"29","1046":"0","3224":"3.148","3466":"29","865":"1","1045":"0","3223":"3.146","3465":"29","866":"0","867":"1","1066":"0","3002":"1.84","3244":"3.182","3486":"29","1065":"0","3001":"0","3243":"3.18","3485":"29","1064":"0","3000":"2","3242":"3.177","3484":"30","1063":"0","3241":"3.179","3483":"29","1062":"1","3240":"3.176","3482":"29","3481":"29","3480":"30","10019":"60","10018":"2","3239":"3.172","3238":"3.179","3237":"3.178","3479":"30","3236":"3.18","3478":"30","3235":"3.176","3477":"29","3234":"3.176","3476":"29","10011":"100","10010":"5","10013":"1","10015":"29","10014":"4","10017":"17","10016":"2","10020":"5","4102":"75.2","1077":"0","3013":"644.6","3255":"3.172","3497":"29","1076":"0","3012":"800.2","3254":"3.179","3496":"29","1075":"0","3011":"3.05","3253":"3.182","3495":"29","1074":"0","3010":"3.5","3252":"3.186","3494":"29","1073":"0","3251":"3.187","3493":"29","1072":"0","3250":"3.184","3492":"29","1071":"0","3491":"29","1070":"0","3490":"29","3009":"5","10029":"25","3008":"95","3007":"0","3249":"3.182","3006":"0","3248":"3.177","1069":"0","3005":"0","3247":"3.184","3489":"29","1068":"0","3004":"0","3246":"3.183","3488":"29","1067":"0","3003":"0","3245":"3.182","3487":"29","10022":"5","10021":"50","10023":"100","10026":"784","10028":"806.4","10027":"795.2","1080":"0","4112":"0","4113":"0","1088":"0","3266":"3.149","1087":"0","3265":"3.148","1086":"0","3264":"3.148","1085":"0","3263":"3.145","1084":"0","3262":"3.142","1083":"0","3261":"3.145","1082":"0","3260":"3.15","1081":"0","3018":"CC00V4.1.0.20231108","3017":"V3.2.3.20230928","3259":"3.18","3016":"V3.0.0.20230224","3258":"3.18","1079":"0","3015":"291","3257":"3.182","3499":"29","1078":"0","3014":"3","3256":"3.175","3498":"29","4103":"0","4104":"0","4105":"8.8","1091":"0","1090":"0","4120":"0","4121":"0","4122":"0","4123":"0","4124":"0","1099":"0","3277":"3.151","1098":"0","3276":"3.157","1097":"0","3275":"3.147","1096":"0","3274":"3.141","1095":"0","3273":"3.141","1094":"0","3272":"3.142","1093":"0","3271":"3.135","1092":"0","3270":"3.143","10008":"5","10007":"60","10009":"50","3269":"3.143","3268":"3.147","1089":"0","3267":"3.153","4114":"0","4115":"0","4116":"0","10002":"4","4117":"0","10001":"1","4118":"0","10004":"2","4119":"0","10003":"29","10006":"2","10005":"17","3280":"3.153","4130":"0","4131":"0","4132":"0","4133":"0","4134":"0","3288":"3.159","3287":"3.162","3286":"3.164","3285":"3.165","3284":"3.158","3283":"3.155","3282":"3.15","3281":"3.161","3279":"3.133","3278":"3.153","4125":"0","4126":"0","4127":"0","4128":"0","4129":"0","3291":"3.157","3290":"3.162","3057":"0","3299":"3.143","3056":"50.04","3298":"3.14","3055":"0","3297":"3.134","3054":"0","3296":"3.137","3053":"0","3295":"3.139","3052":"225","3294":"3.142","3051":"226.1","3293":"3.134","3050":"225","3292":"3.142","200":"0","201":"0","202":"0","203":"0","204":"0","205":"0","3289":"3.156","206":"0","207":"0","4137":"0","208":"0","209":"0","3060":"0","3068":"0","3067":"0","3066":"0","3065":"0","3064":"0","3063":"0","3062":"0","3061":"0","210":"0","211":"0","212":"0","213":"0","214":"1","215":"0","3059":"0","216":"0","3058":"0","217":"1","218":"0","219":"1","3071":"1","3070":"1","3079":"41562.136","3078":"39104.144","3077":"44196.88","3076":"33","3075":"0","3074":"704","3073":"0","3072":"1","220":"1","221":"0","222":"1","223":"0","224":"0","225":"0","226":"0","227":"0","3069":"1","228":"0","229":"0","3082":"0","3081":"0","3080":"39742.364","QA-gjtsgzz":"1","3089":"28","3088":"180","3087":"48059","3086":"0","3085":"0","3084":"0","230":"0","3083":"0","231":"0","232":"0","233":"0","234":"0","235":"0","236":"0","237":"1","3093":"706.7","3092":"0","3091":"0","3090":"6","3099":"30","3098":"3.103","3097":"3.19","3096":"6","3095":"11","3094":"0","3902":"20","3903":"5","3904":"3","3905":"0","3906":"0","3907":"2","3908":"90","3909":"95","3900":"15","3901":"18","3913":"56.8","3914":"57.6","3915":"4","3916":"49.6","3917":"46.4","3918":"44.8","3919":"4","3910":"100","3911":"5","3912":"56","3704":"8","3946":"0","3705":"9","3947":"227.6","3706":"8","3948":"0.96","3707":"7","3949":"0","3708":"10","3709":"11","3941":"30.8","3700":"10","3942":"227.6","3701":"8","3943":"0","3702":"8","3944":"0","3703":"7","3945":"227.6","3715":"10","3957":"49.99","3716":"11","3958":"0","3717":"10","3959":"0","3718":"10","3719":"11","3950":"0","3951":"0.192","3710":"10","3952":"-0.072","3711":"11","3953":"0.204","3712":"11","3954":"0.931","3713":"11","3955":"0","3714":"11","3956":"0","3970":"915.42","3726":"11","3968":"253.8","3727":"11","3969":"0","3728":"10","3729":"11","3960":"31.5","3961":"31.7","3720":"11","3962":"31.6","3721":"11","3963":"0","3722":"11","3964":"2546.1","3723":"10","3965":"2546.1","3724":"11","3966":"0","3725":"8","3967":"316.26","3980":"0","3737":"8","3979":"0","3738":"8","3739":"8","3971":"789.12","3730":"10","3972":"788.94","3731":"9","3973":"0","3732":"11","3974":"915.42","3733":"10","3975":"789.12","3734":"8","3976":"788.94","3735":"9","3977":"0","3736":"6","3978":"0","3990":"394.1","3991":"394.4","3750":"6","3992":"394.3","3505":"29","3748":"8","3504":"29","3749":"6","3503":"29","3502":"29","3501":"29","3500":"29","3740":"8","3741":"6","3742":"6","3743":"8","3509":"29","3744":"6","3986":"29.1","3508":"29","3745":"8","3987":"227.5","3507":"29","3746":"6","3988":"227.6","3506":"29","3747":"6","3989":"227.9","3760":"7","3761":"7","3516":"29","3759":"7","3515":"29","3514":"29","3513":"29","3512":"30","3511":"29","3510":"29","3751":"6","3993":"2.4","3752":"6","3994":"2.8","3753":"6","3995":"2.4","3754":"6","3996":"0","3755":"7","3997":"1.84","3519":"29","3756":"8","3998":"1.84","3518":"29","3757":"7","3999":"0.021","3517":"29","3758":"7","3770":"7","3771":"8","3772":"31","3530":"65495","3527":"65495","3526":"65495","3525":"65495","3524":"65495","3523":"65495","3522":"65495","3521":"65495","3520":"65495","3762":"6","3763":"6","3764":"6","3765":"7","3766":"7","3767":"7","3529":"65495","3768":"7","3528":"65495","3769":"7","3541":"65495","3540":"65495","3538":"65495","3537":"65495","3536":"65495","3535":"65495","3534":"65495","3533":"65495","3532":"65495","931":"1","3531":"65495","932":"1","3773":"31","933":"1","3774":"30","934":"0","3775":"170","935":"0","3776":"13.182","936":"0","3777":"31","937":"0","3539":"65495","3790":"1","3791":"0","3792":"0","3793":"0","3794":"0","3310":"3.164","3552":"7","3551":"8","3550":"6","logTime":1714838402000,"3307":"3.143","3549":"8","3306":"3.141","3548":"7","3305":"3.141","3547":"65495","3304":"3.139","3546":"65495","3303":"3.13","3545":"65495","3302":"3.14","3544":"65495","3301":"3.145","3543":"65495","3300":"3.145","3542":"65495","3309":"3.17","3308":"3.171","3321":"3.181","3563":"8","3320":"3.18","3562":"8","3561":"7","3560":"8","3318":"3.182","3317":"3.184","3559":"8","3316":"3.19","3558":"8","3315":"3.176","3557":"8","3314":"3.169","3556":"8","3313":"3.17","3555":"6","952":"1","3312":"3.169","3554":"6","953":"0","3311":"3.171","3553":"8","954":"0","3795":"28.5","955":"0","3796":"27.5","956":"0","3797":"29.3","957":"0","3798":"78","958":"0","3799":"29","959":"0","3319":"3.176","3332":"3.169","3574":"10","3331":"3.164","3573":"11","3330":"3.158","3572":"10","3571":"10","3570":"11","3329":"3.158","3328":"3.159","3327":"3.159","3569":"11","960":"0","3326":"3.162","3568":"11","961":"0","3325":"3.16","3567":"11","962":"0","3324":"3.166","3566":"11","963":"0","3323":"3.179","3565":"10","964":"0","3322":"3.175","3564":"11","965":"0","966":"0","967":"0","968":"0","969":"0","3101":"100","3343":"3.158","3585":"11","3100":"28","3342":"3.159","3584":"11","3341":"3.159","3583":"11","3340":"3.159","3582":"11","3581":"11","3580":"11","3339":"3.165","970":"0","3338":"3.163","971":"0","3337":"3.163","3579":"11","972":"0","3336":"3.163","3578":"11","973":"0","3335":"3.164","3577":"10","974":"0","3334":"3.163","3576":"10","975":"0","3333":"3.164","3575":"9","976":"0","977":"0","978":"0","979":"0","3112":"-13.3","3354":"3.158","3596":"8","3111":"-13.3","3353":"3.157","3595":"11","3110":"50","3352":"3.156","3594":"11","3351":"3.156","3593":"10","3350":"3.158","3592":"10","3591":"10","3590":"11","3109":"230","980":"0","3108":"0","981":"0","3107":"0","3349":"3.151","982":"0","3106":"-40","3348":"3.16","983":"0","3105":"0","3347":"3.162","3589":"11","984":"0","3104":"0","3346":"3.161","3588":"10","985":"0","3103":"3","3345":"3.159","3587":"11","986":"0","3102":"50","3344":"3.161","3586":"11","987":"0","988":"0","989":"0","3123":"180","3365":"3.156","3122":"180","3364":"3.162","3121":"600","3363":"3.157","3120":"810","3362":"3.139","3361":"3.147","3360":"3.155","3119":"220","3118":"220","3117":"220","3359":"3.151","3116":"0","3358":"3.153","3115":"0","3357":"3.151","3599":"8","3114":"0","3356":"3.15","3598":"8","997":"1","3113":"-13.3","3355":"3.158","3597":"6","998":"0","999":"0","3376":"3.181","3134":"32","3375":"3.182","3133":"32","3374":"3.18","3132":"33","3373":"3.178","3131":"32","3372":"3.183","3130":"61.1","3371":"3.155","3370":"3.155","3129":"0","3128":"0","3127":"0","3369":"3.153","3126":"0","3368":"3.156","3125":"0","3367":"3.15","3124":"0","3366":"3.155","3387":"3.181","3386":"3.179","3385":"3.179","3384":"3.18","3383":"3.179","3382":"3.178","3381":"3.181","3380":"3.183","10118":"180","10117":"180","10119":"0","3138":"33","3379":"3.181","3137":"33","3378":"3.181","3136":"33","3377":"3.177","3135":"33","10110":"0","10112":"220","10111":"0","10114":"220","10113":"220","10116":"600","10115":"810","3390":"3.169","4000":"0","4001":"0","4002":"49.99","4003":"0","3398":"3.157","3397":"3.166","3396":"3.173","3395":"3.173","3394":"3.169","3393":"3.169","3392":"3.169","3391":"3.174","3389":"3.165","3388":"3.171","10121":"0","10120":"0","10123":"0","10122":"0","10124":"0","4010":"37788.4","4011":"41299.2","4012":"8983.6","4013":"0","4014":"0","3167":"1","3166":"1","3165":"3","3164":"16.879","3163":"99","3162":"6","3161":"0","3160":"706.7","3399":"3.162","4004":"0","4005":"29.1","4006":"29.1","4007":"29.1","4008":"0","4009":"79087.6","3170":"3","4020":"308.8","4021":"30","4022":"0","4023":"255.2","4024":"11264.4","4025":"29175.6","3178":"3.19","3177":"3.156","3176":"224","3175":"29","3174":"16","3173":"3","3172":"28","3171":"14","10107":"-13.3","10106":"-13.3","10109":"0","10108":"-13.3","3169":"30","3168":"84","4015":"37142.8","4016":"11573.2","4017":"29205.6","10101":"-40","4018":"0","10100":"0","4019":"36887.6","10103":"0","10102":"0","10105":"50","10104":"230","3181":"3.103","3180":"105","4031":"29","4032":"8","4033":"299","4034":"0","3189":"1","3188":"6","3187":"17","3186":"2","3185":"11","3184":"9","3183":"204","3182":"13","3179":"7","3192":"1","3191":"31","3190":"3","3199":"2","3198":"50.8","3197":"30","3196":"3","3195":"1","3194":"30","3193":"1","100":"0","101":"0","102":"0","103":"0","104":"0","105":"0","106":"0","107":"0","108":"0","109":"0","4052":"0","4053":"0","4054":"227.3","chargeState":"3","4055":"227.4","4056":"227.7","4057":"393.7","4058":"394.1","110":"0","111":"0","112":"0","113":"0","114":"0","115":"0","116":"0","117":"0","118":"0","10099":"0","119":"0","10098":"3","11":"1","4062":"136","12":"0","13":"0","14":"0","15":"0","16":"0","4067":"96","17":"1","18":"1","19":"1","4060":"136","120":"0","4061":"144","121":"0","122":"0","123":"0","124":"0","125":"0","126":"0","127":"0","128":"0","129":"0","4059":"394","20":"1","21":"1","22":"1","23":"0","24":"0","4075":"97.6","10071":"0","25":"0","10070":"3","26":"0","10073":"2","27":"0","10072":"0","28":"0","4079":"0.988","10075":"95","29":"1","10074":"90","130":"0","4071":"-14.4","131":"0","132":"0","133":"0","134":"0","135":"0","136":"0","137":"0","138":"0","139":"0","10077":"5","10076":"100","10079":"56.8","10078":"56","30":"0","31":"0","32":"1","33":"0","4084":"96","10080":"57.6","4085":"0","4086":"86400","10082":"49.6","10081":"4","10084":"44.8","10083":"46.4","10085":"4","140":"0","4081":"49.98","141":"0","4082":"812816","142":"0","4083":"812720","143":"0","144":"0","145":"0","146":"0","147":"0","148":"0","149":"0","10051":"0","10050":"5","10053":"5","10052":"-5","150":"0","151":"0","152":"0","153":"0","154":"0","155":"0","156":"0","157":"0","158":"0","159":"0","10055":"3.6","10054":"3.55","10057":"0.2","10056":"3.7","10059":"2.8","10058":"3","10060":"2.5","10062":"0.3","10061":"0.2","10064":"1","10063":"0.5","cus-b001":"3","160":"0","161":"0","162":"0","163":"0","cus-b002":"1","164":"0","cus-b003":"95.808","165":"0","166":"0","10066":"15","10065":"0.2","10068":"20","61":"1","10067":"18","62":"0","63":"0","10069":"5","64":"0","65":"0","66":"0","67":"0","68":"1","69":"1","10031":"649.6","10030":"694.4","171":"1","172":"0","173":"0","174":"0","175":"0","176":"0","177":"0","178":"0","179":"0","10033":"25","70":"0","10032":"627.2","71":"0","10035":"180","72":"0","10034":"155","73":"0","10037":"10","74":"1","10036":"200","75":"0","10039":"180","76":"1","10038":"155","77":"0","78":"0","79":"0","10040":"200","10042":"0","10041":"10","180":"0","181":"0","182":"0","183":"0","184":"0","185":"0","186":"0","187":"0","188":"0","189":"0","80":"0","10044":"500","81":"0","10043":"0","82":"0","10046":"50","83":"0","10045":"100","84":"0","10048":"60","85":"0","10047":"55","86":"0","87":"0","10049":"5","88":"0","89":"0","190":"0","191":"0","192":"0","193":"0","194":"0","195":"0","196":"0","197":"0","198":"0","199":"0","90":"0","91":"0","92":"0","93":"0","94":"0","95":"0","96":"0","97":"0","98":"0","99":"0","3803":"60","3804":"5","3805":"50","3806":"5","3807":"100","3808":"1","3809":"4","3800":"2","3801":"17","3802":"2","3810":"29","3811":"78.9","3825":"28.5","3826":"27.7","3827":"29.2","3828":"78","3829":"29","3820":"1","3821":"0","3822":"0","3823":"0","3824":"0","3836":"5","3837":"100","3838":"1","3839":"4","3830":"2","3831":"17","3832":"2","3833":"60","3834":"5","3835":"50","3604":"7","3603":"6","3602":"7","3601":"8","3600":"7","3840":"29","3841":"78.9","3609":"7","3608":"7","3607":"8","3606":"6","3605":"6","3860":"784","3615":"6","3858":"255","3614":"8","3859":"0","3613":"6","3612":"8","3611":"8","3610":"6","3850":"78.9","3851":"28.6","3852":"88.5","3853":"26.4","3619":"6","3618":"7","3617":"11","3856":"4","3616":"8","3857":"0","3870":"200","3871":"10","3626":"8","3869":"180","3625":"7","3624":"7","3623":"8","3622":"8","3621":"9","3620":"9","3861":"795.2","3862":"806.4","3863":"25","3864":"694.4","3865":"649.6","3629":"7","3866":"627.2","3628":"7","3867":"25","3627":"8","3868":"155"},"dataType":1,"deviceKey":"CN-18b08cf88f1","deviceSn":"CN-18b08cf88f1","productKey":"8sffV8oVNAd","timestamp":1714838402000}"#;

    if let Ok(res) = parse_log(input) {
        println!("{:?}", res.1);
    } else {
        println!("{}", input);
    }
}
