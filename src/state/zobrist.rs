use std::fmt::{self, Display};

use crate::{
    mv::castle::Castle,
    piece_type::PieceType,
    side::{Side, SIDE_MAP},
    square::Square,
};

use super::{castle_rights::CastleRights, position::Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Zobrist(pub u64);
impl Zobrist {
    fn to_u64(self) -> u64 {
        self.0
    }

    pub fn hash_piece(&mut self, side: Side, piece_type: PieceType, sq: Square) {
        self.0 ^= PIECES_KEY[side.to_usize()][piece_type.to_usize()][sq.to_usize()];
    }

    pub fn hash_en_passant(&mut self, en_passant: Option<Square>) {
        if let Some(sq) = en_passant {
            self.0 ^= EN_PASSANT_KEY[sq.file()];
        }
    }

    pub fn hash_side(&mut self, side: Side) {
        if side == Side::Black {
            self.0 ^= SIDE_KEY;
        }
    }

    pub fn hash_castle_rights_all(&mut self, castle_rights: CastleRights) {
        for bit_set in castle_rights.iter() {
            self.0 ^= CASTLE_RIGHTS_KEY[bit_set];
        }
    }

    pub fn hash_castle_rights_single(&mut self, side: Side, castle: Castle) {
        let side_start_idx: usize = if side == Side::White { 2 } else { 0 };
        let castle_idx: usize = if castle == Castle::Kingside { 1 } else { 0 };
        let castle_rights_idx = castle_idx + side_start_idx;
        self.0 ^= CASTLE_RIGHTS_KEY[castle_rights_idx];
    }

    pub fn new(
        position: &Position,
        castle_rights: CastleRights,
        en_passant: Option<Square>,
        side: Side,
    ) -> Zobrist {
        let mut zobrist = Zobrist(0);
        for sq in 0..BOARD_SIZE {
            let sq = Square::new(sq);
            let pc_result = position.at(sq);
            if let Some(pc) = pc_result {
                let side = pc.side();
                let piece_type = pc.piece_type();

                zobrist.hash_piece(side, piece_type, sq);
            }
        }

        let castle_map: [Castle; 2] = [Castle::Queenside, Castle::Kingside];
        for side in SIDE_MAP {
            for castle in castle_map {
                if castle_rights.can(side, castle) {
                    zobrist.hash_castle_rights_single(side, castle);
                }
            }
        }

        let en_passant_result = en_passant;
        zobrist.hash_en_passant(en_passant_result);

        zobrist.hash_side(side);

        zobrist
    }
}

impl Display for Zobrist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
pub mod test_zobrist {
    use crate::state::castle_rights;

    use super::*;

    #[test]
    fn test_hash_castle_rights_w_kingside() {
        let mut zobrist = Zobrist(0);
        zobrist.hash_castle_rights_single(Side::White, Castle::Kingside);

        assert_eq!(zobrist.to_u64(), CASTLE_RIGHTS_KEY[3])
    }

    #[test]
    fn test_hash_castle_rights_w_queenside() {
        let mut zobrist = Zobrist(0);
        zobrist.hash_castle_rights_single(Side::White, Castle::Queenside);

        assert_eq!(zobrist.to_u64(), CASTLE_RIGHTS_KEY[2])
    }

    #[test]
    fn test_hash_castle_rights_b_kingside() {
        let mut zobrist = Zobrist(0);
        zobrist.hash_castle_rights_single(Side::Black, Castle::Kingside);

        assert_eq!(zobrist.to_u64(), CASTLE_RIGHTS_KEY[1])
    }

    #[test]
    fn test_hash_castle_rights_b_queenside() {
        let mut zobrist = Zobrist(0);
        zobrist.hash_castle_rights_single(Side::Black, Castle::Queenside);

        assert_eq!(zobrist.to_u64(), CASTLE_RIGHTS_KEY[0])
    }

    #[test]
    fn test_hash_castle_rights_all_1() {
        let mut zobrist = Zobrist(0);
        let castle_rights = castle_rights::ALL;

        zobrist.hash_castle_rights_all(castle_rights);

        assert_eq!(
            zobrist.0,
            CASTLE_RIGHTS_KEY[0]
                ^ CASTLE_RIGHTS_KEY[1]
                ^ CASTLE_RIGHTS_KEY[2]
                ^ CASTLE_RIGHTS_KEY[3]
        )
    }

    #[test]
    fn test_hash_castle_rights_all_2() {
        let mut zobrist = Zobrist(0);
        let castle_rights = castle_rights::WHITE;

        zobrist.hash_castle_rights_all(castle_rights);
        zobrist.hash_castle_rights_all(castle_rights);

        assert_eq!(zobrist.0, 0)
    }
}

const BOARD_SIZE: usize = 64;
const NUM_UNIQUE_PIECES: usize = 6;
const NUM_SIDES: usize = 2;
const NUM_FILES: usize = 8;
const NUM_CASTLE_RIGHTS: usize = 4;

const PIECES_KEY: [[[u64; BOARD_SIZE]; NUM_UNIQUE_PIECES]; NUM_SIDES] = [
    [
        [
            13349407626915607332,
            3741067114410992119,
            12833997290456451957,
            6313377312852357883,
            4939959359000992263,
            7287542763429058408,
            17293409372962767718,
            7788957729243932100,
            12278218265720379107,
            6867287596311794741,
            848761400353135248,
            13306800623285509231,
            179543212350332426,
            8121482374700224004,
            5139132759758389384,
            16772251437892896608,
            584535241277571194,
            11886555542273930395,
            3095448441507515093,
            18129716037143458919,
            7304849465977000056,
            3231686698239431950,
            15139506225287868027,
            11028302075513709818,
            6967104860862043418,
            17335874815030886561,
            6765466790261748856,
            5878007491081908957,
            736544984217051992,
            18123597996688899705,
            16092294621917404308,
            17438247512610517259,
            9595025519634524535,
            15645709120282115080,
            5796738128751169660,
            2961489217673866164,
            7383868557355071696,
            16309039814870159299,
            2660953274254675430,
            13923311445340179115,
            15532998864486832085,
            14791185622955844387,
            9290805005322213271,
            5912855415308844257,
            11039416316519897039,
            9666653407433484801,
            4606679632345838100,
            11290260570984519754,
            13351868131948075171,
            7296492023402663298,
            2992157368403301147,
            11009929644104265706,
            8575430311312651406,
            17012230364499151707,
            4252852148995863969,
            11323255795355511630,
            15915092555076367310,
            3961969241085911601,
            585031155423405459,
            10832720978446847199,
            15623402267474725764,
            7211903222514247891,
            12478644671491746838,
            17505408767777679671,
        ],
        [
            12667286158370647292,
            5517917543956120187,
            9664404591252632723,
            13007795298867981461,
            12985985166125290399,
            2543164554715486406,
            10218548972789515986,
            2831437895461273286,
            2479491465621673767,
            3042875828012510445,
            12711701207047993766,
            17469504781099823958,
            13028449895801547215,
            17096565571150670185,
            11422048003240416816,
            11747836858594831717,
            4468200225748212547,
            16798065362518615102,
            3473914674000538794,
            6331797211192445396,
            15514837818258340864,
            8614426992932073376,
            11477191674009257820,
            7822806868131693611,
            6621637965457310462,
            1069818642842922760,
            13637750469789479368,
            8654571880743471206,
            13426757441028277183,
            2614751451003693010,
            2211442651268420883,
            11249627355252502637,
            11893642946824405580,
            13489623252815918694,
            9039199800295024071,
            14327447153067706195,
            3200644242742552045,
            1881374189197931152,
            12731558273593635720,
            4721393976913628846,
            18444869848688242363,
            3389565343317296241,
            372390903684722678,
            15448729393141400084,
            7972668244131779905,
            8487233360822286260,
            8495805318436616207,
            2750471071237572655,
            2050211765970727713,
            2426037432335437224,
            4300587492656749211,
            17810317714501752820,
            14722572692900541869,
            10697501819496384537,
            9507157880884258941,
            15763117172734559377,
            16680172657045047798,
            8699850599049991295,
            16450132562493427271,
            4198267277686520120,
            16457437114067139090,
            2995731810764292148,
            4040036813509785620,
            8707223878910189023,
        ],
        [
            4754166857795485009,
            7844565939075339173,
            6662111421416879335,
            7909926386599776027,
            13613109989689921660,
            665950053581044589,
            9831316997108082581,
            5735616133352487964,
            17888389789031029895,
            17116270614721360717,
            6307497842569277928,
            8344041786825186155,
            11156333124226373888,
            15541918022550906649,
            6626579595009263668,
            4048370594990484908,
            14023834143098912731,
            14629050327052227189,
            13690492886631777361,
            16095692977741056948,
            1396175759828057771,
            12263630686199881724,
            16430791715910707385,
            7048308275959050789,
            3923398776243586017,
            2971390277621831349,
            15567177600775014423,
            5706504804248902991,
            16186342078434812871,
            9238569872684966776,
            10438164606538902548,
            782424481317316786,
            2515533256940615001,
            2894288600728124229,
            1504023353170742975,
            11198268684098019433,
            12929769206568400715,
            14054163379561747268,
            16965895130598466753,
            18206613652320522063,
            13307020766295758066,
            11378221824978440727,
            4137970175927786257,
            10246068422377515247,
            12832748976569358910,
            4370703526495182120,
            8771657661868796273,
            17289597858179987561,
            10005362106022854891,
            3000548763292833240,
            11918611167126215549,
            8303933792323796720,
            2373067320832560486,
            15261891161301242628,
            6662206484262549509,
            2488293286252831240,
            3632378963522003345,
            16282357935376863277,
            13494275332863584100,
            16072489448736053428,
            7164406664331372535,
            15417685849385886451,
            15424918171218205343,
            31944480995494250,
        ],
        [
            11375138782991761341,
            10775864598599444694,
            2042564347737640249,
            18070609900275254627,
            10449620863035246274,
            13914447119630704558,
            9554845954329620735,
            14621557154120418259,
            8445320345103978553,
            9874212092345290367,
            3606215637586334998,
            16623272689077632184,
            12283527913326851298,
            14244117483026345409,
            16329853813109029223,
            9993085421674055121,
            16587926937138052453,
            10017218583413339290,
            12547552645477189480,
            14350433805258202661,
            7999074864247533152,
            1293180315481515394,
            14814769547857152214,
            14168283270756576753,
            10248292424829460099,
            14708674468395442734,
            14087227282959665544,
            9025221428156755322,
            10757045420491988977,
            2442235845380050487,
            10777928718931820965,
            17501758362740818329,
            2388668254200544528,
            1013499051946522386,
            2771242845997940912,
            6032882685956572418,
            15082938852156118434,
            14463587526999180472,
            8415454970227064871,
            12377204538794088224,
            26085369799183860,
            5276303674800447828,
            9831338725720772698,
            4821680897744652930,
            8246687644424440910,
            5384712783954444597,
            2866069093783131318,
            4660378132289741448,
            2565334314040973221,
            16709855919925265503,
            2229713165246146605,
            10359543819606585048,
            12706764007941478428,
            17447083789563185960,
            4614457883618707892,
            4628082924498831059,
            2598578616712296822,
            15737742714781566136,
            10085174445588101631,
            3620345022874628403,
            3268174895305427897,
            10218117170283150063,
            18200794945946395447,
            4009915069097979610,
        ],
        [
            13729860914420538605,
            7451790739174563194,
            15028620279206487181,
            10040008254135648275,
            5374957360830079991,
            12830200750780008906,
            13333913143738938874,
            3320904268949052514,
            9363568742026182570,
            8179404878408163424,
            5427707159396255571,
            16001210504416439939,
            3298367609434778787,
            16150924202133312020,
            10744463925100114406,
            16682022169090708328,
            3722535627883273534,
            1017008032464647065,
            17270807912030005452,
            16908103320766635968,
            14610314707594081773,
            2603462513149643154,
            8364616225850832591,
            3006843309823909422,
            2041831745930728511,
            9051372841900971183,
            17719490500693667066,
            15648566204790821963,
            7211913849214894475,
            12150710731794677965,
            7303936762241482574,
            7099055266291220223,
            6721896312621643733,
            602689364923926718,
            882181810259543956,
            1369469729442237896,
            16732462633378768608,
            15618614296141299168,
            7415499736822357305,
            5477275617650849154,
            17445022165768496965,
            18337861562894230664,
            17253098735406284786,
            13601603589470634620,
            14132747390952946346,
            6460495999510835240,
            10947863211842246140,
            3188845260543055330,
            7577260634463288048,
            4988972120710229222,
            1444596016649994741,
            9785618312396207879,
            7980270085472331182,
            7440169990782468781,
            15078046312615451473,
            11998796873357723986,
            18204410743206279400,
            7625342338800303753,
            10105133883136052081,
            4329419645582699165,
            6089099895594617382,
            12208165094502116387,
            1907921915913542974,
            17079795852622701838,
        ],
        [
            4793438742632578875,
            13647716971582507850,
            8263201219993640070,
            140168800919328932,
            8342375185740720851,
            5520540642160456363,
            13199421537360551935,
            13391764726833224156,
            1475616338041932697,
            15868819713754208502,
            11692525055386250475,
            15129533433255657561,
            10056192329972251584,
            11487600192446764354,
            8622685910050615348,
            14858377481260362728,
            6819511614496321859,
            18329266553494810214,
            849296188859321400,
            8719464333662821501,
            1302167344166835959,
            7512854444056740192,
            10087565905448608141,
            1139389104348225991,
            2536860734823674209,
            8091872988900407554,
            13666726768666209900,
            2287194885103993248,
            15209618408059458724,
            2359903144126657402,
            1953108208488529692,
            13691141343253841319,
            15406047981320385360,
            14041320478412416864,
            10869735347398136034,
            1042959362277879766,
            15176489792185158094,
            11283658386629072177,
            7688274441700057970,
            6356954437315243605,
            3420718368008352346,
            6207237750566819492,
            1444800506025890986,
            8789407018531650183,
            2882544566553865103,
            10796985587334860285,
            3122294806329263489,
            17067672956514355328,
            9590591059447033858,
            7962899964927119343,
            1059393307213031556,
            3114616766814337013,
            4149246061141212362,
            4714575483082153121,
            12243049454194325146,
            15462803358671671659,
            2289420626780814699,
            12608003444155031131,
            13156265260765459279,
            8382498500336004410,
            10491801955805790266,
            9947188790880132012,
            16396073552574209384,
            15765515741842153245,
        ],
    ],
    [
        [
            8334078415922684898,
            10666975049204191768,
            8878698937151663191,
            6771511984481634572,
            15311382315565682786,
            13363398324188673266,
            13035191572630425985,
            15660138909055393592,
            16513060502546921024,
            8673599761969631005,
            1169857851297125935,
            818412426191111653,
            17248507063992478223,
            4759351103002916171,
            6630143961195118931,
            15942277606421606353,
            13271601514612536868,
            12335221976173315608,
            6194856989593378152,
            16740755079587224074,
            11765749590857304734,
            136767460085479122,
            1355199727433359781,
            5876448179288316653,
            5167437149140302840,
            15091187851770806906,
            12832306265809086217,
            15501749366615518537,
            10515220925594704663,
            11347494180963291525,
            17554178301008811675,
            5084558405589387007,
            14303932825595842731,
            9958950394753627568,
            15388721773884854782,
            9001919205505459181,
            15490302106208279616,
            13426347519516564672,
            9882344861413285033,
            14697961188679374730,
            8124404531647078103,
            2738030495886126177,
            15573512492255902407,
            16001649961166748551,
            17267571210394358287,
            16195310963997381751,
            14530733813073592348,
            7402560888087745204,
            558502713847153924,
            962452178709980158,
            11759204507095937625,
            4992675117991405962,
            8605280512985366744,
            1724952147306229654,
            13097174402283786061,
            7696338008185184874,
            5365710154262486416,
            14086201421319954314,
            15350406736217687261,
            10292519486417751484,
            10228371096439996120,
            6041194229383900419,
            6925776646405845926,
            15558738447904301691,
        ],
        [
            2152072338022962745,
            12998801299741223892,
            10990325508362284267,
            5887574705311615644,
            14077966540476034292,
            15072606621154935543,
            7095784490099238767,
            5264727114242188324,
            6646902490191882040,
            2002948949095275537,
            6443192048969311525,
            6983352581774341674,
            11470275679765246557,
            7341176346785488197,
            10661124460379938097,
            10611634202016698578,
            11708175275351751401,
            1158848249133769068,
            7104896552290103355,
            11492477589350909328,
            14472325340336846979,
            8906868743380843474,
            10172866008531289211,
            7177682084273719393,
            3532103598413428423,
            4641343826085415361,
            15998876736431884045,
            3223707069902442471,
            12027455890829385654,
            4763874008261122545,
            13947973524641955872,
            14196946057776738993,
            9194725016075004146,
            12566751190599060367,
            3969756095141697815,
            4308246291874334436,
            343741742607916214,
            10552327111873184876,
            6778404237183491523,
            453187899980168664,
            4400704647922648058,
            14963771172690738803,
            601028352557054678,
            16944263911871371970,
            18436191935723670740,
            17951091441666025904,
            5962234098333334135,
            14175859903115908448,
            3829510523503280596,
            1043339474311895444,
            3840132364397647683,
            6928353401858768217,
            9613849443197565574,
            16183756936441200466,
            14447386818959586096,
            11771144269507221482,
            9208479911489955454,
            1213667395665916985,
            17815860496441698726,
            15613325049665567698,
            6701172272674339563,
            9811841012448519629,
            996725360976416815,
            8072152249475581580,
        ],
        [
            17358934492862777570,
            17564725310315217439,
            14702420927620245540,
            10379411074569843599,
            5683983018902831333,
            17309241922489160167,
            16539355920089803668,
            10677331074761541382,
            7326085970068175592,
            16660182983169444398,
            15950319208481997984,
            16424018672655018484,
            5764118417564600931,
            16523744411180756348,
            2994701881229305383,
            7886152963549362700,
            9899913849135979651,
            16825406809596538918,
            11290533819560814303,
            11016564006315373286,
            10420249297244249576,
            16789921833699981500,
            3536433598652947553,
            7495808880143040037,
            16496098253146164779,
            4546983864741836420,
            6758612009059687561,
            14044354506911295187,
            4636314466829106861,
            97655003389489358,
            17595571831371790974,
            14726246400679796967,
            10133626157071030907,
            4856304650924392991,
            10557421257037329190,
            6263986010444051285,
            4479805898553751341,
            10749605105673547820,
            9276882401807653627,
            7000547071292654535,
            3043322635777570206,
            6820911935180900548,
            15987539419966303870,
            3263426003330649378,
            17039861079453930913,
            7494759721093247332,
            5339525262508809870,
            4500929126769458927,
            16927239596004043911,
            5214180319070728716,
            2307039966707790025,
            7517337264852994784,
            4726032684556535500,
            10334973243099367549,
            4329997901798230300,
            7478566019613375695,
            11285933419791355554,
            15160707792169255288,
            9401362500441653166,
            4435114367273324100,
            13015263533199017780,
            2667521411143121302,
            9191877996536026892,
            2673451986072267820,
        ],
        [
            8910967455760632700,
            10145066645964293226,
            840979938496545175,
            11034953229507180774,
            16988238114280430947,
            15453875603272474097,
            16301824208111042709,
            15525216566578912921,
            15584916671788440513,
            9624355145070256046,
            12238553195175088085,
            724347494140120510,
            3503645230141244021,
            6925630785988318245,
            6391392449607119418,
            754639357843271557,
            15169147609390382646,
            7081969815323554525,
            2208516393603896181,
            14533049269354003209,
            5563759433344498660,
            9751626676813227341,
            18229374221019563326,
            16334395662816248731,
            12416927195599802393,
            993658775511670336,
            14830711812611260636,
            272702641556468863,
            11087938450059994661,
            14159557938983681867,
            6584799581330963061,
            8834587286251302949,
            15724829096965890822,
            16045563744288104596,
            60246553690797577,
            1111327619227071179,
            7386115728925093008,
            14431905130561690894,
            10893185084599509683,
            16187019910402892781,
            5047673890455351369,
            17132144865767019871,
            18107203869490708282,
            11599536358555568799,
            11795028358858136807,
            14208456780079247482,
            16187205724380846303,
            12210252834458856995,
            5704624667978156735,
            3209266181572594216,
            15500486050443178662,
            14073007354982941774,
            10456360324400882081,
            3912036400928272617,
            3352477504069953607,
            3124152295694351992,
            789501229638922019,
            10263229835755816122,
            11893958425196268656,
            1410807044401125005,
            15361967863575582294,
            12628632763211907851,
            6876692543120303577,
            9799285939184768063,
        ],
        [
            5696824604191176750,
            9861622512977208251,
            1263711262032565630,
            6151867608563154228,
            802365059956414591,
            6115248125564807286,
            16110731916404754237,
            15020707377374071908,
            3981720035939043068,
            14863695307540744572,
            8287008795851369937,
            14807542177442317657,
            4194108197296975579,
            8541869724888500142,
            3898514703794095323,
            12308343507446847664,
            15009310526992729683,
            1481111014644273483,
            1695609215848616353,
            872112709127795713,
            4926024542609452990,
            842552072954536898,
            14592337272233536373,
            6273037149031720030,
            610831433905073084,
            15800215827847596380,
            12278688404762984507,
            15466449456789822787,
            15994444134577336320,
            7423648786252040944,
            17515041286901311342,
            3893434989272745616,
            11340114928595388007,
            12356093833453867040,
            9726261004465123115,
            4770578464951805792,
            10420675091887610668,
            2621365010901212102,
            11496794391913368582,
            18178871415872062322,
            1035945327410617486,
            1203074221280055058,
            7037678244114657157,
            15720572011051287275,
            6935033474841239625,
            5996585713767441931,
            8631670034161415487,
            8794637374564074347,
            15038396450611084606,
            13207195595631705359,
            2899517314967363671,
            3328462832439880325,
            13846160052409560777,
            13734998413380917470,
            14939642421868594071,
            13071894191823732297,
            4878160015048414746,
            16632964818957807981,
            9233254712086623192,
            3827897615453419386,
            17188031026981613096,
            12511149305241901091,
            16550425843367028802,
            15257243388621873929,
        ],
        [
            8461569999310337659,
            11327134239513488192,
            16964419654847036006,
            12042415649128044488,
            13441924591695252954,
            4561635917634080778,
            2796452258916024427,
            15608534868471654492,
            13382568885335787564,
            2400740727255413134,
            3174227177988971246,
            7249964013909178561,
            17538608134377812045,
            12472556522100251430,
            17439909788640519738,
            8597885872074720824,
            10016346377956589268,
            1786248142767117462,
            439014977915133835,
            14675844838705265443,
            9634380260935796786,
            8348955666002548502,
            10008682498562807642,
            12146195609324519090,
            13175848275092679999,
            6589422737857279041,
            15232977328786598219,
            7670454464182746350,
            13275853604207534166,
            720692052565769345,
            17196138055571095745,
            2387416980173524504,
            15205758445660995488,
            13192733919179035755,
            1528943483277500270,
            8537052689976652424,
            14466707669577168098,
            3182095549483599811,
            13421724641962141663,
            11270943141919369957,
            11328835108703142488,
            8867610905818338064,
            2965718671383149924,
            6034536600555807117,
            2854343812763999722,
            7280474502759477945,
            16001977944287365277,
            5072520801349400067,
            7304616744946767321,
            10540659330472448030,
            16985026393036599768,
            16527217069224187948,
            16860885696609508719,
            751839899539201183,
            11534341083948253023,
            14607844588515306668,
            12446763294527990547,
            15158217776162013822,
            7175217419986559114,
            12494702894307935359,
            723045844742466405,
            12118820415997957694,
            7637463583388092360,
            4990770468204784652,
        ],
    ],
];

const EN_PASSANT_KEY: [u64; NUM_FILES] = [
    14445015829651790644,
    6862386483721547573,
    1605628974285353569,
    7992613458119180510,
    15857307622962714835,
    2192276730872831606,
    9698014204117960897,
    14651024747477140607,
];

const CASTLE_RIGHTS_KEY: [u64; NUM_CASTLE_RIGHTS] = [
    9544595991968925137,
    6723252973147187608,
    8164052353413222815,
    4832131808676571486,
];

const SIDE_KEY: u64 = 14998782759734077275;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
fn generate_key() {
    use rand::RngCore;

    let mut rng = rand::thread_rng();
    let mut pieces_key: [[[u64; BOARD_SIZE]; NUM_UNIQUE_PIECES]; NUM_SIDES] =
        [[[0; BOARD_SIZE]; NUM_UNIQUE_PIECES]; NUM_SIDES];
    let mut en_passant_key: [u64; NUM_FILES] = [0; NUM_FILES];
    let mut castle_rights_key: [u64; NUM_CASTLE_RIGHTS] = [0; NUM_CASTLE_RIGHTS];
    let side_key = rng.next_u64();

    for side_idx in 0..NUM_SIDES {
        for piece_idx in 0..NUM_UNIQUE_PIECES {
            for sq_idx in 0..BOARD_SIZE {
                pieces_key[side_idx][piece_idx][sq_idx] = rng.next_u64();
            }
        }
    }

    for file_idx in 0..NUM_FILES {
        en_passant_key[file_idx] = rng.next_u64();
    }

    for castle_rights_idx in 0..NUM_CASTLE_RIGHTS {
        castle_rights_key[castle_rights_idx] = rng.next_u64();
    }

    println!("pieces: {:?}", pieces_key);
    println!("ep: {:?}", en_passant_key);
    println!("castle: {:?}", castle_rights_key);
    println!("side: {:?}", side_key);
}

#[cfg(test)]
pub mod print_zobrist {
    use super::*;

    #[ignore = "only needed to print zobrist key"]
    #[test]
    fn print() {
        generate_key();
    }
}
