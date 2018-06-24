pub const PERM: [i32; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CELL_2D_X : [f32;256] = 
[
	-0.6440658039f32, -0.08028078721, 0.9983546168, 0.9869492062, 0.9284746418, 0.6051097552, -0.794167404, -0.3488667991, -0.943136526, -0.9968171318, 0.8740961579, 0.1421139764, 0.4282553608, -0.9986665833, 0.9996760121, -0.06248383632,
	0.7120139305, 0.8917660409, 0.1094842955, -0.8730880804, 0.2594811489, -0.6690063346, -0.9996834972, -0.8803608671, -0.8166554937, 0.8955599676, -0.9398321388, 0.07615451399, -0.7147270565, 0.8707354457, -0.9580008579, 0.4905965632,
	0.786775944, 0.1079711577, 0.2686638979, 0.6113487322, -0.530770584, -0.7837268286, -0.8558691039, -0.5726093896, -0.9830740914, 0.7087766359, 0.6807027153, -0.08864708788, 0.6704485923, -0.1350735482, -0.9381333003, 0.9756655376,
	0.4231433671, -0.4959787385, 0.1005554325, -0.7645857281, -0.5859053796, -0.9751154306, -0.6972258572, 0.7907012002, -0.9109899213, -0.9584307894, -0.8269529333, 0.2608264719, -0.7773760119, 0.7606456974, -0.8961083758, -0.9838134719,
	0.7338893576, 0.2161226729, 0.673509891, -0.5512056873, 0.6899744332, 0.868004831, 0.5897430311, -0.8950444221, -0.3595752773, 0.8209486981, -0.2912360132, -0.9965011374, 0.9766994634, 0.738790822, -0.4730947722, 0.8946479441,
	-0.6943628971, -0.6620468182, -0.0887255502, -0.7512250855, -0.5322986898, 0.5226295385, 0.2296318375, 0.7915307344, -0.2756485999, -0.6900234522, 0.07090588086, 0.5981278485, 0.3033429312, -0.7253142797, -0.9855874307, -0.1761843396,
	-0.6438468325, -0.9956136595, 0.8541580762, -0.9999807666, -0.02152416253, -0.8705983095, -0.1197138014, -0.992107781, -0.9091181546, 0.788610536, -0.994636402, 0.4211256853, 0.3110430857, -0.4031127839, 0.7610684239, 0.7685674467,
	0.152271555, -0.9364648723, 0.1681333739, -0.3567427907, -0.418445483, -0.98774778, 0.8705250765, -0.8911701067, -0.7315350966, 0.6030885658, -0.4149130821, 0.7585339481, 0.6963196535, 0.8332685012, -0.8086815232, 0.7518116724,
	-0.3490535894, 0.6972110903, -0.8795676928, -0.6442331882, 0.6610236811, -0.9853565782, -0.590338458, 0.09843602117, 0.5646534882, -0.6023259233, -0.3539248861, 0.5132728656, 0.9380385118, -0.7599270056, -0.7425936564, -0.6679610562,
	-0.3018497816, 0.814478266, 0.03777430269, -0.7514235086, 0.9662556939, -0.4720194901, -0.435054126, 0.7091901235, 0.929379209, 0.9997434357, 0.8306320299, -0.9434019629, -0.133133759, 0.5048413216, 0.3711995273, 0.98552091,
	0.7401857005, -0.9999981398, -0.2144033253, 0.4808624681, -0.413835885, 0.644229305, 0.9626648696, 0.1833665934, 0.5794129, 0.01404446873, 0.4388494993, 0.5213612322, -0.5281609948, -0.9745306846, -0.9904373013, 0.9100232252,
	-0.9914057719, 0.7892627765, 0.3364421659, -0.9416099764, 0.7802732656, 0.886302871, 0.6524471291, 0.5762186726, -0.08987644664, -0.2177026782, -0.9720345052, -0.05722538858, 0.8105983127, 0.3410261032, 0.6452309645, -0.7810612152,
	0.9989395718, -0.808247815, 0.6370177929, 0.5844658772, 0.2054070861, 0.055960522, -0.995827561, 0.893409165, -0.931516824, 0.328969469, -0.3193837488, 0.7314755657, -0.7913517714, -0.2204109786, 0.9955900414, -0.7112353139,
	-0.7935008741, -0.9961918204, -0.9714163995, -0.9566188669, 0.2748495632, -0.4681743221, -0.9614449642, 0.585194072, 0.4532946061, -0.9916113176, 0.942479587, -0.9813704753, -0.6538429571, 0.2923335053, -0.2246660704, -0.1800781949,
	-0.9581216256, 0.552215082, -0.9296791922, 0.643183699, 0.9997325981, -0.4606920354, -0.2148721265, 0.3482070809, 0.3075517813, 0.6274756393, 0.8910881765, -0.6397771309, -0.4479080125, -0.5247665011, -0.8386507094, 0.3901291416,
	0.1458336921, 0.01624613149, -0.8273199879, 0.5611100679, -0.8380219841, -0.9856122234, -0.861398618, 0.6398413916, 0.2694510795, 0.4327334514, -0.9960265354, -0.939570655, -0.8846996446, 0.7642113189, -0.7002080528, 0.664508256,
    ];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CELL_2D_Y : [f32;256]  =
[
	0.7649700911f32, 0.9967722885, 0.05734160033, -0.1610318741, 0.371395799, -0.7961420628, 0.6076990492, -0.9371723195, 0.3324056156, 0.07972205329, -0.4857529277, -0.9898503007, 0.9036577593, 0.05162417479, -0.02545330525, -0.998045976,
	-0.7021653386, -0.4524967717, -0.9939885256, -0.4875625128, -0.9657481729, -0.7432567015, 0.02515761212, 0.4743044842, 0.5771254669, 0.4449408324, 0.3416365773, 0.9970960285, 0.6994034849, 0.4917517499, 0.286765333, 0.8713868327,
	0.6172387009, 0.9941540269, 0.9632339851, -0.7913613129, 0.847515538, 0.6211056739, 0.5171924952, -0.8198283277, -0.1832084353, 0.7054329737, 0.7325597678, 0.9960630973, 0.7419559859, 0.9908355749, -0.346274329, 0.2192641299,
	-0.9060627411, -0.8683346653, 0.9949314574, -0.6445220433, -0.8103794704, -0.2216977607, 0.7168515217, 0.612202264, -0.412428616, 0.285325116, 0.56227115, -0.9653857009, -0.6290361962, 0.6491672535, 0.443835306, -0.1791955706,
	-0.6792690269, -0.9763662173, 0.7391782104, 0.8343693968, 0.7238337389, 0.4965557504, 0.8075909592, -0.4459769977, -0.9331160806, -0.5710019572, 0.9566512346, -0.08357920318, 0.2146116448, -0.6739348049, 0.8810115417, 0.4467718167,
	-0.7196250184, -0.749462481, 0.9960561112, 0.6600461127, -0.8465566164, -0.8525598897, -0.9732775654, 0.6111293616, -0.9612584717, -0.7237870097, -0.9974830104, -0.8014006968, 0.9528814544, -0.6884178931, -0.1691668301, 0.9843571905,
	0.7651544003, -0.09355982605, -0.5200134429, -0.006202125807, -0.9997683284, 0.4919944954, -0.9928084436, -0.1253880012, -0.4165383308, -0.6148930171, -0.1034332049, -0.9070022917, -0.9503958117, 0.9151503065, -0.6486716073, 0.6397687707,
	-0.9883386937, 0.3507613761, 0.9857642561, -0.9342026446, -0.9082419159, 0.1560587169, 0.4921240607, -0.453669308, 0.6818037859, 0.7976742329, 0.9098610522, 0.651633524, 0.7177318024, -0.5528685241, 0.5882467118, 0.6593778956,
	0.9371027648, -0.7168658839, -0.4757737632, 0.7648291307, 0.7503650398, 0.1705063456, -0.8071558121, -0.9951433815, -0.8253280792, -0.7982502628, 0.9352738503, 0.8582254747, -0.3465310238, 0.65000842, -0.6697422351, 0.7441962291,
	-0.9533555, 0.5801940659, -0.9992862963, -0.659820211, 0.2575848092, 0.881588113, -0.9004043022, -0.7050172826, 0.369126382, -0.02265088836, 0.5568217228, -0.3316515286, 0.991098079, -0.863212164, -0.9285531277, 0.1695539323,
	-0.672402505, -0.001928841934, 0.9767452145, -0.8767960349, 0.9103515037, -0.7648324016, 0.2706960452, -0.9830446035, 0.8150341657, -0.9999013716, -0.8985605806, 0.8533360801, 0.8491442537, -0.2242541966, -0.1379635899, -0.4145572694,
	0.1308227633, 0.6140555916, 0.9417041303, -0.336705587, -0.6254387508, 0.4631060578, -0.7578342456, -0.8172955655, -0.9959529228, -0.9760151351, 0.2348380732, -0.9983612848, 0.5856025746, -0.9400538266, -0.7639875669, 0.6244544645,
	0.04604054566, 0.5888424828, 0.7708490978, -0.8114182882, 0.9786766212, -0.9984329822, 0.09125496582, -0.4492438803, -0.3636982357, 0.9443405575, -0.9476254645, -0.6818676535, -0.6113610831, 0.9754070948, -0.0938108173, -0.7029540015,
	-0.6085691109, -0.08718862881, -0.237381926, 0.2913423132, 0.9614872426, 0.8836361266, -0.2749974196, -0.8108932717, -0.8913607575, 0.129255541, -0.3342637104, -0.1921249337, -0.7566302845, -0.9563164339, -0.9744358146, 0.9836522982,
	-0.2863615732, 0.8337016872, 0.3683701937, 0.7657119102, -0.02312427772, 0.8875600535, 0.976642191, 0.9374176384, 0.9515313457, -0.7786361937, -0.4538302125, -0.7685604874, -0.8940796454, -0.8512462154, 0.5446696133, 0.9207601495,
	-0.9893091197, -0.9998680229, 0.5617309299, -0.8277411985, 0.545636467, 0.1690223212, -0.5079295433, 0.7685069899, -0.9630140787, 0.9015219132, 0.08905695279, -0.3423550559, -0.4661614943, -0.6449659371, 0.7139388509, 0.7472809229,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CELL_3D_X: [f32;256]  =
[
	0.3752498686f32, 0.687188096, 0.2248135212, 0.6692006647, -0.4376476931, 0.6139972552, 0.9494563929, 0.8065108882, -0.2218812853, 0.8484661167, 0.5551817596, 0.2133903499, 0.5195126593, -0.6440141975, -0.5192897331, -0.3697654077,
	-0.07927779647, 0.4187757321, -0.750078731, 0.6579554632, -0.6859803838, -0.6878407087, 0.9490848347, 0.5795829433, -0.5325976529, -0.1363699466, 0.417665879, -0.9108236468, 0.4438605427, 0.819294887, -0.4033873915, -0.2817317705,
	0.3969665622, 0.5323450134, -0.6833017297, 0.3881436661, -0.7119144767, -0.2306979838, -0.9398873022, 0.1701906676, -0.4261839496, -0.003712295499, -0.734675004, -0.3195046015, 0.7345307424, 0.9766246496, -0.02003735175, -0.4824156342,
	0.4245892007, 0.9072427669, 0.593346808, -0.8911762541, -0.7657571834, -0.5268198896, -0.8801903279, -0.6296409617, -0.09492481344, -0.4920470525, 0.7307666154, -0.2514540636, -0.3356210347, -0.3522787894, 0.87847885, -0.7424096346,
	0.5757585274, 0.4519299338, 0.6420368628, -0.1128478447, 0.499874883, 0.5291681739, -0.5098837195, 0.5639583502, -0.8456386526, -0.9657134875, -0.576437342, -0.5666013014, 0.5667702405, -0.481316582, 0.7313389916, -0.3805628566,
	-0.6512675909, -0.2787156951, 0.8648059114, -0.9730216276, -0.8335820906, 0.2673159641, 0.231150148, 0.01286214638, 0.6774953261, 0.6542885718, -0.02545450161, 0.2101238586, -0.5572105885, 0.813705672, -0.7546026951, -0.2502500006,
	-0.9979289381, 0.7024037039, 0.08990874624, 0.8170812432, 0.4226980265, -0.2442153475, -0.9183326731, 0.6068222411, 0.818676691, -0.7236735282, -0.5383903295, -0.6269337242, -0.0939331121, 0.9203878539, -0.7256396824, 0.6292431149,
	0.4234156978, 0.006685688024, -0.2598694113, 0.6408036421, 0.05899871622, 0.7090281418, -0.5905222072, 0.3128214264, -0.691925826, 0.3634019349, -0.6772511147, -0.3204583896, -0.3906740409, -0.3342190395, -0.517779592, -0.6817711267,
	0.6422383105, 0.4388482478, 0.2968562611, -0.2019778353, 0.6014865048, 0.9519280722, 0.3398889569, 0.8179709354, 0.2365522154, 0.3262175096, -0.8060715954, -0.2068642503, 0.6208057279, -0.5274282502, -0.3722334928, -0.8923412971,
	0.5341834201, -0.3663701513, -0.6114600319, 0.5026307556, 0.8396151729, 0.9245042467, -0.7994843957, -0.5357200589, -0.6283359739, -0.61351886, -0.875632008, -0.5278879423, 0.9087491985, -0.03500215466, -0.261365798, -0.579523541,
	-0.3765052689, -0.74398252, 0.4257318052, -0.1214508921, 0.8561809753, 0.6802835104, -0.5452131039, -0.1997156478, 0.4562348357, -0.811704301, 0.67793962, -0.9237819106, 0.6973511259, -0.5189506, 0.5517320032, -0.396710831,
	0.5493762815, -0.2507853002, 0.4788634005, 0.387333516, -0.2176515694, 0.6749832419, 0.2148283022, -0.7521815872, 0.4697000159, 0.7890593699, -0.7606162952, 0.01083397843, 0.5254091908, -0.6748025877, 0.751091524, 0.05259056135,
	0.01889481232, -0.6037423727, -0.6542965129, 0.08873301081, -0.6191345671, 0.4331858488, -0.3858351946, -0.1429059747, 0.4118221036, -0.6247153214, -0.611423014, 0.5542939606, -0.9432768808, -0.4567870451, -0.7349133547, 0.399304489,
	-0.7474927672, 0.02589419753, 0.783915821, 0.6138668752, 0.4276376047, -0.4347886353, 0.02947841302, -0.833742746, 0.3817221742, -0.8743368359, -0.3823443796, -0.6829243811, -0.3681903049, -0.367626833, -0.434583373, 0.235891995,
	-0.6874880269, -0.5115661773, -0.5534962601, 0.5632777056, 0.686191532, -0.05095871588, -0.06865785057, -0.5975288531, -0.6429790056, -0.3729361548, 0.2237917666, 0.6046773225, -0.5041542295, -0.03972191174, 0.7028828406, -0.5560856498,
	0.5898328456, -0.9308076766, 0.4617069864, 0.3190983137, 0.9116567753, -0.45029554, 0.3346334459, 0.8525005645, 0.2528483381, -0.8306630147, -0.6880390622, 0.7448684026, -0.1963355843, -0.5900257974, 0.9097057294, -0.2509196808,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CELL_3D_Y: [f32;256]  =
[
	-0.6760585049f32, -0.09136176499, 0.1681325679, -0.6688468686, -0.4822753902, -0.7891068824, -0.1877509944, 0.548470914, -0.463339443, -0.4050542082, 0.3218158513, 0.2546493823, -0.3753271935, 0.4745384887, 0.481254652, -0.8934416489,
	-0.6737085076, 0.7469917228, 0.3826230411, 0.6751013678, -0.7248119515, -0.3224276742, -0.02076190936, -0.6404268166, -0.5292028444, 0.7151414636, -0.6144655059, -0.369912124, 0.6942067212, -0.4481558248, -0.6366894559, 0.5956568471,
	0.564274539, 0.7145584688, 0.6871918316, 0.5657918509, -0.6275978114, 0.4146983062, 0.2638993789, -0.792633138, 0.5706133514, 0.8606546462, 0.6490900316, -0.8242699196, 0.6765819124, 0.1959534069, -0.8426769757, -0.5917672797,
	0.7517364266, 0.03252559226, 0.0883617105, 0.4475064813, -0.1418643552, 0.7343428473, 0.3870192548, -0.7716703522, 0.4839898327, 0.7437439055, -0.5989573348, -0.8357068955, 0.6086049038, 0.9194627258, 0.4718297238, -0.2650335884,
	-0.6470352599, -0.5555181303, 0.1222351235, 0.7802044684, -0.8636947022, -0.2341352163, 0.683030874, -0.5005858287, 0.2334616211, 0.2576877608, 0.6666816727, -0.7663996863, 0.794201982, 0.6189308788, 0.6071033261, -0.4206058253,
	-0.3957336915, -0.8170257484, -0.1043240417, 0.0002167596213, 0.1816339018, -0.6838094939, -0.2495341969, -0.7116756954, -0.03361673621, -0.3350836431, 0.2137186039, 0.2557996786, 0.7490117093, 0.4942936549, -0.352686853, -0.3952445435,
	-0.0459964767, -0.7115787471, 0.08022899756, 0.5362268157, -0.8258613686, 0.1114171723, 0.3882823051, -0.7915404457, 0.3250957662, 0.6401346464, -0.2662724517, -0.6727907114, -0.994730818, -0.3596358977, 0.2344610069, -0.6645215546,
	-0.7107590611, -0.4646617327, 0.6717191355, 0.5101893498, 0.1185768238, 0.236005093, -0.7811024061, 0.5089325193, 0.6073187658, -0.7930732557, -0.6822767155, 0.3201532885, 0.7545302807, 0.1072664448, 0.6784033173, -0.6595924967,
	0.7276509498, 0.5586689436, -0.6498636788, 0.6789333174, 0.7105966551, -0.2872214155, 0.496746217, -0.3880337977, 0.7324070604, -0.9326634749, -0.5867839255, 0.8003043651, -0.1631882481, -0.6796374681, -0.8066678503, 0.4238177418,
	0.7715863549, 0.5455367347, -0.03205115397, -0.6005545066, -0.5423640002, 0.3569205906, -0.582071752, 0.6407354361, 0.7777142984, -0.09956428618, 0.1100002681, 0.8136349123, 0.2923431904, 0.9735794425, 0.8324974864, -0.6179617717,
	-0.9248386523, -0.6448780771, -0.5274402761, -0.7862170565, 0.2682099744, -0.5848777694, -0.6364561467, -0.7167402514, -0.8677012494, 0.4205286707, -0.7007832749, 0.243272451, -0.1899846085, -0.6146124977, -0.8093357692, -0.03545096987,
	-0.7191590868, 0.7478645848, 0.3623517328, 0.8436992512, -0.2445711729, 0.6897356637, -0.1708070787, 0.4639272368, -0.7917186656, 0.02980025428, 0.6334156172, -0.9815544807, -0.2307217304, 0.1080823318, 0.5167601798, -0.845120016,
	0.441572562, 0.5876789172, -0.6365908737, 0.68350166, 0.5849723959, 0.1164114357, -0.7379813884, -0.9613237178, -0.9071943084, -0.7682111105, 0.639074459, -0.619358298, 0.2807257131, -0.01800868791, 0.3776607289, 0.7207567823,
	0.5536661486, -0.9974053117, -0.02047200006, -0.6739453804, -0.5607471297, 0.8815553192, 0.8275977415, 0.3928902456, 0.550991396, 0.4247623676, -0.3436948871, -0.3653537677, 0.3181702902, -0.6067173171, -0.8984128477, 0.4220839766,
	0.7238407199, -0.7766913695, 0.6460037842, 0.2544775664, 0.6488840578, 0.805016833, -0.9183807036, 0.4144046357, 0.270587208, -0.8813684494, 0.6985971877, -0.7795603017, -0.8624480731, 0.5532697017, 0.711179521, -0.7798160574,
	0.5225859041, 0.1261859368, 0.3398033582, -0.7472173667, -0.4032647119, -0.4246578154, 0.8481212377, -0.2144838537, 0.3431714491, 0.5310188231, 0.6682978632, 0.3110433206, 0.9263293599, -0.6155600569, 0.07169784399, 0.8985888773,
];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CELL_3D_Z: [f32;256]  =
[
	-0.6341391283f32, -0.7207118346, 0.9597866014, 0.3237504235, -0.7588642466, -0.01782410481, 0.2515593809, 0.2207257205, -0.8579541106, 0.3406410681, 0.7669470462, -0.9431957648, 0.7676171537, -0.6000491115, -0.7062096948, 0.2550207115,
	0.7347325213, 0.5163625202, -0.5394270162, 0.3336656285, -0.0638635111, -0.6503195787, 0.3143356798, -0.5039217245, 0.6605180464, -0.6855479011, -0.6693185756, 0.1832083647, -0.5666258437, 0.3576482138, -0.6571949095, -0.7522101635,
	-0.7238865886, 0.4538887323, 0.2467106257, 0.7274778869, 0.3151170655, -0.8802293764, -0.2167232729, 0.5854637865, 0.7019741052, 0.5091756071, 0.1973189533, 0.46743546, 0.05197599597, 0.088354718, 0.5380464843, -0.6458224544,
	-0.5045952393, 0.419347884, 0.8000823542, -0.07445020656, -0.6272881641, -0.428020311, -0.2747382083, -0.08987283726, 0.8699098354, 0.4524761885, -0.3274603257, 0.4882262167, -0.7189983256, 0.1746079907, 0.0751772698, -0.6152927202,
	0.4998474673, -0.6979677227, 0.7568667263, -0.6152612058, 0.06447140991, -0.8155744872, -0.5229602449, 0.6567836838, -0.4799905631, 0.03153534591, 0.4724992466, -0.3026458097, -0.2191225827, -0.620692287, 0.3107552588, 0.8235670294,
	0.6474915988, -0.5047637941, 0.4911488878, -0.2307138167, -0.5216800015, 0.6789305939, 0.9403734863, 0.702390397, 0.7347584625, 0.6779567958, 0.9765635805, -0.9436177661, -0.358465925, -0.3058706624, 0.5533414464, -0.8838306897,
	0.04496841812, 0.01687374963, -0.9927133148, -0.211752318, 0.3732015249, 0.9632990593, -0.07682417004, -0.07232213047, 0.4733721775, 0.2579229713, 0.7995216286, 0.3928189967, 0.04107517667, 0.1534542912, 0.6468965045, 0.4030684878,
	-0.5617300988, -0.885463029, 0.693729985, -0.5736527866, -0.9911905409, -0.66451538, 0.2028855685, 0.8019541421, -0.3903877149, -0.4888495114, -0.2753714057, -0.8915202143, 0.5273119089, 0.9363714773, -0.5212228249, -0.31642672,
	0.2409440761, -0.703776404, -0.6996810411, -0.7058714505, -0.3650566783, 0.1064744278, 0.7985729102, 0.424680257, -0.6384535592, 0.1540161646, -0.07702731943, -0.5627789132, -0.7667919169, -0.509815999, 0.4590525092, 0.1552595611,
	0.345402042, 0.7537656024, 0.7906259247, -0.6218493452, 0.02979350071, -0.1337893489, -0.1483818606, 0.549965562, 0.01882482408, -0.7833783002, 0.4702855809, 0.2435827372, 0.2978428332, 0.2256499906, 0.4885036897, 0.5312962584,
	0.05401156992, 0.1749922158, -0.7352273018, 0.6058980284, 0.4416079111, 0.4417378638, 0.5455879807, -0.6681295324, 0.1973431441, 0.4053292055, 0.2220375492, 0.2957118467, 0.6910913512, 0.5940890106, -0.2014135283, -0.9172588213,
	-0.4254361401, -0.6146586825, -0.7996193253, -0.3716777111, -0.9448876842, -0.2620349924, 0.9615995749, -0.4679683524, 0.3905937144, 0.613593722, 0.1422937358, 0.1908754211, 0.8189704912, -0.7300408736, -0.4108776451, -0.5319834504,
	-0.8970265651, -0.5386359045, 0.4082255906, 0.7245356676, 0.5239080873, -0.8937552226, -0.553637673, 0.2354455182, -0.0860293075, -0.1399373318, -0.4666323327, 0.5560157407, 0.1772619533, -0.8893937725, -0.5632714576, -0.5666264959,
	-0.3670263736, -0.06717242579, 0.6205295181, -0.4110536264, 0.7090054553, 0.183899597, -0.5605470555, 0.3879565548, 0.7420893903, -0.2347595118, -0.8577217497, 0.6325590203, -0.8736152276, 0.7048011129, -0.06317948268, 0.8753285574,
	-0.05843650473, -0.3674922622, -0.5256624401, 0.7861039337, 0.3287714416, 0.5910593099, -0.3896960134, 0.6864605361, 0.7164918431, -0.290014277, -0.6796169617, 0.1632515592, 0.04485347486, 0.8320545697, 0.01339408056, -0.2874989857,
	0.615630723, 0.3430367014, 0.8193658136, -0.5829600957, 0.07911697781, 0.7854296063, -0.4107442306, 0.4766964066, -0.9045999527, -0.1673856787, 0.2828077348, -0.5902737632, -0.321506229, -0.5224513133, -0.4090169985, -0.3599685311,
];
