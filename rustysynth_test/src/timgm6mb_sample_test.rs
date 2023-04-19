#![allow(unused_imports)]

use rustysynth::SoundFont;
use std::fs::File;
use std::path::PathBuf;

use crate::sample_util;

#[test]
fn samples() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("TimGM6mb.sf2");
    let mut file = File::open(&path).unwrap();
    let sf = SoundFont::new(&mut file).unwrap();

    // FluteG6
    let values: [i32; 7] = [0, 9320, 3924, 7954, 22500, 79, 43];
    sample_util::check(&sf.get_sample_headers()[0], &values);

    // FluteA#6
    let values: [i32; 7] = [9352, 22108, 17929, 22086, 22500, 82, 47];
    sample_util::check(&sf.get_sample_headers()[1], &values);

    // FluteB7
    let values: [i32; 7] = [22140, 32262, 27982, 31880, 22500, 95, -21];
    sample_util::check(&sf.get_sample_headers()[2], &values);

    // FluteC#6
    let values: [i32; 7] = [32294, 45270, 41064, 45256, 22500, 73, 33];
    sample_util::check(&sf.get_sample_headers()[3], &values);

    // FluteD#7
    let values: [i32; 7] = [45302, 53339, 49229, 53327, 22500, 87, 46];
    sample_util::check(&sf.get_sample_headers()[4], &values);

    // FluteD5
    let values: [i32; 7] = [53371, 63368, 58637, 62875, 22500, 62, 49];
    sample_util::check(&sf.get_sample_headers()[5], &values);

    // FluteE5
    let values: [i32; 7] = [63400, 78940, 74639, 78815, 22500, 64, 49];
    sample_util::check(&sf.get_sample_headers()[6], &values);

    // FluteF8
    let values: [i32; 7] = [78972, 88246, 85302, 88227, 22500, 101, 30];
    sample_util::check(&sf.get_sample_headers()[7], &values);

    // FluteG5
    let values: [i32; 7] = [88278, 100550, 96382, 100530, 22500, 67, 31];
    sample_util::check(&sf.get_sample_headers()[8], &values);

    // FluteA#5
    let values: [i32; 7] = [100582, 116509, 112327, 116476, 22500, 69, -47];
    sample_util::check(&sf.get_sample_headers()[9], &values);

    // whistle
    let values: [i32; 7] = [116541, 122400, 121936, 122392, 44642, 79, 0];
    sample_util::check(&sf.get_sample_headers()[10], &values);

    // HarpG#5
    let values: [i32; 7] = [122432, 123379, 123268, 123371, 44100, 68, -50];
    sample_util::check(&sf.get_sample_headers()[11], &values);

    // HarpF6
    let values: [i32; 7] = [123411, 124011, 123940, 124003, 44100, 77, -1];
    sample_util::check(&sf.get_sample_headers()[12], &values);

    // HarpD#7
    let values: [i32; 7] = [124043, 124965, 124922, 124957, 44100, 87, -18];
    sample_util::check(&sf.get_sample_headers()[13], &values);

    // StringW
    let values: [i32; 7] = [124997, 125355, 125337, 125347, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[14], &values);

    // IceRain
    let values: [i32; 7] = [125387, 156471, 130152, 156459, 17857, 60, 0];
    sample_util::check(&sf.get_sample_headers()[15], &values);

    // OceanWaves
    let values: [i32; 7] = [156503, 192873, 175208, 192838, 22500, 78, 50];
    sample_util::check(&sf.get_sample_headers()[16], &values);

    // Bird
    let values: [i32; 7] = [192905, 206623, 199764, 206622, 11025, 41, 0];
    sample_util::check(&sf.get_sample_headers()[17], &values);

    // TrumpC5
    let values: [i32; 7] = [206655, 224498, 220322, 224419, 22050, 60, -16];
    sample_util::check(&sf.get_sample_headers()[18], &values);

    // TrumpD#5
    let values: [i32; 7] = [224530, 250660, 246288, 250660, 22050, 63, -9];
    sample_util::check(&sf.get_sample_headers()[19], &values);

    // TrumpG5
    let values: [i32; 7] = [250692, 271027, 266840, 271026, 22050, 67, -10];
    sample_util::check(&sf.get_sample_headers()[20], &values);

    // TrumpA#5
    let values: [i32; 7] = [271059, 289182, 285157, 289061, 22050, 70, -8];
    sample_util::check(&sf.get_sample_headers()[21], &values);

    // TrumpC#6
    let values: [i32; 7] = [289214, 309878, 305546, 309877, 22050, 73, -19];
    sample_util::check(&sf.get_sample_headers()[22], &values);

    // TrumpD#6
    let values: [i32; 7] = [309910, 339653, 334956, 339652, 22050, 75, -3];
    sample_util::check(&sf.get_sample_headers()[23], &values);

    // TrumpG6
    let values: [i32; 7] = [339685, 351570, 349600, 351512, 22050, 79, -1];
    sample_util::check(&sf.get_sample_headers()[24], &values);

    // musicbox
    let values: [i32; 7] = [351602, 366908, 360141, 366901, 22321, 60, 7];
    sample_util::check(&sf.get_sample_headers()[25], &values);

    // Shakuhachi
    let values: [i32; 7] = [366940, 382314, 382281, 382309, 22050, 79, 0];
    sample_util::check(&sf.get_sample_headers()[26], &values);

    // Oboe C#
    let values: [i32; 7] = [382346, 409355, 404485, 409350, 22050, 61, 0];
    sample_util::check(&sf.get_sample_headers()[27], &values);

    // Oboe F#
    let values: [i32; 7] = [409387, 435737, 431255, 435732, 22050, 66, 0];
    sample_util::check(&sf.get_sample_headers()[28], &values);

    // Oboe A#
    let values: [i32; 7] = [435769, 454895, 450290, 454890, 22050, 70, 0];
    sample_util::check(&sf.get_sample_headers()[29], &values);

    // Oboe E5
    let values: [i32; 7] = [454927, 480476, 476381, 480471, 22050, 76, 0];
    sample_util::check(&sf.get_sample_headers()[30], &values);

    // Oboe A5
    let values: [i32; 7] = [480508, 504073, 499809, 504068, 22050, 81, 0];
    sample_util::check(&sf.get_sample_headers()[31], &values);

    // Oboe C6
    let values: [i32; 7] = [504105, 524199, 519553, 524194, 22050, 84, 0];
    sample_util::check(&sf.get_sample_headers()[32], &values);

    // Yolo Ob
    let values: [i32; 7] = [524231, 532789, 531593, 532770, 22050, 92, -28];
    sample_util::check(&sf.get_sample_headers()[33], &values);

    // TubeBell
    let values: [i32; 7] = [532821, 568470, 557857, 568453, 22500, 84, 11];
    sample_util::check(&sf.get_sample_headers()[34], &values);

    // Acoustic Bass A51
    let values: [i32; 7] = [568502, 568789, 568769, 568781, 44100, 106, 29];
    sample_util::check(&sf.get_sample_headers()[35], &values);

    // Acoustic Bass A11
    let values: [i32; 7] = [568821, 574032, 573824, 574024, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[36], &values);

    // Acoustic Bass A31
    let values: [i32; 7] = [574064, 575094, 575045, 575086, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[37], &values);

    // Gun Shot
    let values: [i32; 7] = [575126, 579693, 579220, 579691, 11025, 60, 0];
    sample_util::check(&sf.get_sample_headers()[38], &values);

    // Piano Gb5
    let values: [i32; 7] = [579725, 585453, 585280, 585449, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[39], &values);

    // Piano C5
    let values: [i32; 7] = [585485, 593161, 592966, 593157, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[40], &values);

    // Piano Db4
    let values: [i32; 7] = [593193, 606150, 605641, 606146, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[41], &values);

    // Piano Ab3
    let values: [i32; 7] = [606182, 619708, 619530, 619704, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[42], &values);

    // Piano Db3
    let values: [i32; 7] = [619740, 630818, 630352, 630814, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[43], &values);

    // Piano Ab2
    let values: [i32; 7] = [630850, 644566, 641179, 644561, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[44], &values);

    // Piano Db2
    let values: [i32; 7] = [644598, 652164, 649968, 652160, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[45], &values);

    // Piano Gb1
    let values: [i32; 7] = [652196, 661532, 658320, 661528, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[46], &values);

    // Piano D1
    let values: [i32; 7] = [661564, 670903, 669211, 670900, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[47], &values);

    // Brush Snare
    let values: [i32; 7] = [670935, 673387, 670938, 673380, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[48], &values);

    // Cowbell 808
    let values: [i32; 7] = [673419, 674407, 674256, 674400, 44100, 70, 35];
    sample_util::check(&sf.get_sample_headers()[49], &values);

    // 808 Hat
    let values: [i32; 7] = [674439, 679618, 678015, 679610, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[50], &values);

    // 808 Click
    let values: [i32; 7] = [679650, 680452, 679653, 680445, 44100, 48, 50];
    sample_util::check(&sf.get_sample_headers()[51], &values);

    // 808 Snare
    let values: [i32; 7] = [680484, 682864, 680487, 682857, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[52], &values);

    // Sine Wave
    let values: [i32; 7] = [682896, 683035, 682961, 683027, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[53], &values);

    // E Snare
    let values: [i32; 7] = [683067, 687595, 683070, 687588, 44100, 50, -18];
    sample_util::check(&sf.get_sample_headers()[54], &values);

    // Kick Verb
    let values: [i32; 7] = [687627, 689392, 687630, 689385, 44100, 66, 35];
    sample_util::check(&sf.get_sample_headers()[55], &values);

    // Snare Verb
    let values: [i32; 7] = [689424, 696260, 689427, 696253, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[56], &values);

    // Ride Ping
    let values: [i32; 7] = [696292, 704898, 698209, 704890, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[57], &values);

    // ChinaCrashCymb
    let values: [i32; 7] = [704930, 709544, 706242, 709540, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[58], &values);

    // Castanets
    let values: [i32; 7] = [709576, 712420, 709579, 712413, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[59], &values);

    // Bell Tree
    let values: [i32; 7] = [712452, 723261, 722469, 723253, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[60], &values);

    // Sleigh Bells
    let values: [i32; 7] = [723293, 730015, 727862, 730007, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[61], &values);

    // TriangleWave Db5
    let values: [i32; 7] = [730047, 732115, 730379, 732107, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[62], &values);

    // Mute Triangle
    let values: [i32; 7] = [732147, 734409, 732150, 734402, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[63], &values);

    // Quica Downstroke
    let values: [i32; 7] = [734441, 737497, 734444, 737490, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[64], &values);

    // Quica Hi Tone
    let values: [i32; 7] = [737529, 740986, 737532, 740979, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[65], &values);

    // Rosewood Claves
    let values: [i32; 7] = [741018, 743133, 741021, 743126, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[66], &values);

    // Guiro Up
    let values: [i32; 7] = [743165, 750070, 743168, 745723, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[67], &values);

    // Guiro Down
    let values: [i32; 7] = [750102, 753505, 750105, 753498, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[68], &values);

    // Samba Whistle
    let values: [i32; 7] = [753537, 754832, 753644, 754824, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[69], &values);

    // Maracas
    let values: [i32; 7] = [754864, 757297, 754867, 757290, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[70], &values);

    // Cabasa
    let values: [i32; 7] = [757329, 759861, 757332, 759854, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[71], &values);

    // Timbale Strike
    let values: [i32; 7] = [759893, 769098, 768250, 769090, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[72], &values);

    // Timbale Rimshot
    let values: [i32; 7] = [769130, 777857, 777032, 777849, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[73], &values);

    // Low Tumba Tone
    let values: [i32; 7] = [777889, 782950, 782209, 782942, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[74], &values);

    // Quinto Tone
    let values: [i32; 7] = [782982, 789026, 788234, 789018, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[75], &values);

    // QuintoClosedSlap
    let values: [i32; 7] = [789058, 792435, 789061, 792428, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[76], &values);

    // M Bongo Tone
    let values: [i32; 7] = [792467, 795381, 795292, 795373, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[77], &values);

    // E Bongo Rim
    let values: [i32; 7] = [795413, 800212, 795416, 798044, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[78], &values);

    // Vibra Loop
    let values: [i32; 7] = [800244, 802914, 800409, 802906, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[79], &values);

    // Cowbell
    let values: [i32; 7] = [802946, 805300, 804452, 804875, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[80], &values);

    // Brass Tambourine
    let values: [i32; 7] = [805332, 809220, 808229, 809212, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[81], &values);

    // Open High Hat
    let values: [i32; 7] = [809252, 819586, 814767, 819582, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[82], &values);

    // High Hat Foot
    let values: [i32; 7] = [819618, 821916, 819621, 821909, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[83], &values);

    // High Hat Closed
    let values: [i32; 7] = [821948, 824551, 821951, 824544, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[84], &values);

    // Snare 2
    let values: [i32; 7] = [824583, 831386, 824586, 831379, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[85], &values);

    // Claps 808ish
    let values: [i32; 7] = [831418, 835148, 831421, 835141, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[86], &values);

    // Snare 1
    let values: [i32; 7] = [835180, 840580, 835183, 840573, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[87], &values);

    // Rim Shot
    let values: [i32; 7] = [840612, 844271, 840615, 844264, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[88], &values);

    // Ride Bell
    let values: [i32; 7] = [844303, 855765, 853099, 855760, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[89], &values);

    // Bass Drum New
    let values: [i32; 7] = [855797, 857693, 855800, 857686, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[90], &values);

    // Met Click
    let values: [i32; 7] = [857725, 859148, 857728, 859141, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[91], &values);

    // Sinetick
    let values: [i32; 7] = [859180, 859252, 859183, 859244, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[92], &values);

    // Drum Stick
    let values: [i32; 7] = [859284, 861859, 859287, 861852, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[93], &values);

    // Scratch
    let values: [i32; 7] = [861891, 864392, 861894, 864384, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[94], &values);

    // Noise Slap
    let values: [i32; 7] = [864424, 869097, 864427, 869090, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[95], &values);

    // Filter Snap
    let values: [i32; 7] = [869129, 869730, 869132, 869723, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[96], &values);

    // Applause
    let values: [i32; 7] = [869762, 874108, 869762, 874106, 11025, 60, 0];
    sample_util::check(&sf.get_sample_headers()[97], &values);

    // Helicopter
    let values: [i32; 7] = [874140, 878305, 874143, 878297, 44100, 112, -24];
    sample_util::check(&sf.get_sample_headers()[98], &values);

    // Telephone
    let values: [i32; 7] = [878337, 880304, 878410, 880296, 44100, 88, 17];
    sample_util::check(&sf.get_sample_headers()[99], &values);

    // Bird - Sparrow
    let values: [i32; 7] = [880336, 885771, 882637, 885763, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[100], &values);

    // Ocean Wave
    let values: [i32; 7] = [885803, 889463, 885878, 889460, 11025, 41, -1];
    sample_util::check(&sf.get_sample_headers()[101], &values);

    // FretNoise
    let values: [i32; 7] = [889495, 892912, 889498, 892904, 44100, 78, 24];
    sample_util::check(&sf.get_sample_headers()[102], &values);

    // Med Crash Cymbal
    let values: [i32; 7] = [892944, 906176, 899686, 906121, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[103], &values);

    // Syn Drum Wave
    let values: [i32; 7] = [906208, 911488, 911397, 911480, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[104], &values);

    // Acoustic Tom
    let values: [i32; 7] = [911520, 917458, 916884, 917454, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[105], &values);

    // Taiko Drum
    let values: [i32; 7] = [917490, 923918, 922740, 923910, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[106], &values);

    // Wood Block
    let values: [i32; 7] = [923950, 925653, 923953, 925646, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[107], &values);

    // SteelDrum D5
    let values: [i32; 7] = [925685, 927742, 927509, 927734, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[108], &values);

    // SteelDrum D1
    let values: [i32; 7] = [927774, 935445, 934923, 935437, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[109], &values);

    // Agogo Bell
    let values: [i32; 7] = [935477, 940418, 940076, 940410, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[110], &values);

    // Carillon C6
    let values: [i32; 7] = [940450, 944574, 943888, 944566, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[111], &values);

    // Shenai D6
    let values: [i32; 7] = [944606, 944875, 944856, 944867, 44100, 107, -21];
    sample_util::check(&sf.get_sample_headers()[112], &values);

    // Shenai D4
    let values: [i32; 7] = [944907, 945774, 945730, 945766, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[113], &values);

    // Shenai A#3
    let values: [i32; 7] = [945806, 946617, 946564, 946609, 44100, 83, 16];
    sample_util::check(&sf.get_sample_headers()[114], &values);

    // Shenai F#3
    let values: [i32; 7] = [946649, 947282, 947218, 947274, 44100, 79, -4];
    sample_util::check(&sf.get_sample_headers()[115], &values);

    // Shenai D3
    let values: [i32; 7] = [947314, 948011, 947932, 948003, 44100, 75, 7];
    sample_util::check(&sf.get_sample_headers()[116], &values);

    // Shenai C3
    let values: [i32; 7] = [948043, 948665, 948563, 948657, 44100, 70, -7];
    sample_util::check(&sf.get_sample_headers()[117], &values);

    // Bag Drone A1
    let values: [i32; 7] = [948697, 950827, 950619, 950819, 44100, 57, -4];
    sample_util::check(&sf.get_sample_headers()[118], &values);

    // Bagpipe Wave Bb5
    let values: [i32; 7] = [950859, 950890, 950870, 950882, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[119], &values);

    // Bagpipe Wave Bb3
    let values: [i32; 7] = [950922, 951056, 950991, 951048, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[120], &values);

    // BagDroneWave A1
    let values: [i32; 7] = [951088, 951360, 951107, 951352, 44100, 53, -48];
    sample_util::check(&sf.get_sample_headers()[121], &values);

    // Bagpipe Bb5
    let values: [i32; 7] = [951392, 952025, 952005, 952017, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[122], &values);

    // Bagpipe Bb3
    let values: [i32; 7] = [952057, 955056, 954991, 955048, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[123], &values);

    // Kalimba C5
    let values: [i32; 7] = [955088, 956279, 956219, 956271, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[124], &values);

    // Kalimba C3
    let values: [i32; 7] = [956311, 960744, 960568, 960736, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[125], &values);

    // Koto Ab5
    let values: [i32; 7] = [960776, 961333, 961313, 961325, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[126], &values);

    // Koto G3
    let values: [i32; 7] = [961365, 964143, 964077, 964135, 44100, 78, -43];
    sample_util::check(&sf.get_sample_headers()[127], &values);

    // Koto D3
    let values: [i32; 7] = [964175, 966322, 966236, 966314, 44100, 73, -31];
    sample_util::check(&sf.get_sample_headers()[128], &values);

    // Koto G2
    let values: [i32; 7] = [966354, 969546, 969421, 969538, 44100, 66, -29];
    sample_util::check(&sf.get_sample_headers()[129], &values);

    // Shamison A3
    let values: [i32; 7] = [969578, 971897, 971839, 971889, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[130], &values);

    // Shamison Eb3
    let values: [i32; 7] = [971929, 974089, 974010, 974081, 44100, 75, 7];
    sample_util::check(&sf.get_sample_headers()[131], &values);

    // Shamison Bb2
    let values: [i32; 7] = [974121, 976624, 976522, 976616, 44100, 70, -7];
    sample_util::check(&sf.get_sample_headers()[132], &values);

    // Shamison F2
    let values: [i32; 7] = [976656, 979642, 979508, 979634, 44100, 65, -1];
    sample_util::check(&sf.get_sample_headers()[133], &values);

    // Banjo F6
    let values: [i32; 7] = [979674, 979846, 979828, 979838, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[134], &values);

    // Banjo F5
    let values: [i32; 7] = [979878, 980382, 980358, 980374, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[135], &values);

    // Banjo B2
    let values: [i32; 7] = [980414, 985209, 984608, 984709, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[136], &values);

    // Banjo F3
    let values: [i32; 7] = [985241, 988589, 988510, 988581, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[137], &values);

    // Sitar Eb6
    let values: [i32; 7] = [988621, 988887, 988869, 988879, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[138], &values);

    // Sitar C5
    let values: [i32; 7] = [988919, 990053, 989912, 990045, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[139], &values);

    // Sitar Eb3
    let values: [i32; 7] = [990085, 995155, 994632, 995147, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[140], &values);

    // Spectrum D#5
    let values: [i32; 7] = [995187, 1000066, 995250, 1000058, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[141], &values);

    // Solo Vox C7
    let values: [i32; 7] = [1000098, 1001449, 1000250, 1001441, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[142], &values);

    // Solo Vox C4
    let values: [i32; 7] = [1001481, 1012620, 1001595, 1012612, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[143], &values);

    // Square Wave C7
    let values: [i32; 7] = [1012652, 1012688, 1012670, 1012680, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[144], &values);

    // Square Wave C5
    let values: [i32; 7] = [1012720, 1012744, 1012723, 1012736, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[145], &values);

    // Square Wave C4
    let values: [i32; 7] = [1012776, 1012829, 1012779, 1012821, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[146], &values);

    // Square Wave C3
    let values: [i32; 7] = [1012861, 1012956, 1012864, 1012948, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[147], &values);

    // Square Wave C2
    let values: [i32; 7] = [1012988, 1013167, 1012991, 1013159, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[148], &values);

    // Square Wave C1
    let values: [i32; 7] = [1013199, 1013547, 1013202, 1013539, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[149], &values);

    // Ocarina F#6
    let values: [i32; 7] = [1013579, 1013928, 1013910, 1013920, 44100, 109, 12];
    sample_util::check(&sf.get_sample_headers()[150], &values);

    // Ocarina F#4
    let values: [i32; 7] = [1013960, 1016807, 1015398, 1016799, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[151], &values);

    // Bottle Blow
    let values: [i32; 7] = [1016839, 1019561, 1016842, 1019554, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[152], &values);

    // Pan Flute F6
    let values: [i32; 7] = [1019593, 1019886, 1019876, 1019881, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[153], &values);

    // Pan Flute F4
    let values: [i32; 7] = [1019918, 1023152, 1020914, 1023148, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[154], &values);

    // Pan Flute G3
    let values: [i32; 7] = [1023184, 1028410, 1025000, 1028406, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[155], &values);

    // Recorder C6
    let values: [i32; 7] = [1028442, 1028891, 1028872, 1028883, 44100, 107, -22];
    sample_util::check(&sf.get_sample_headers()[156], &values);

    // Recorder C4
    let values: [i32; 7] = [1028923, 1031143, 1031082, 1031135, 44100, 80, 0];
    sample_util::check(&sf.get_sample_headers()[157], &values);

    // Piccolo D6
    let values: [i32; 7] = [1031175, 1032513, 1032493, 1032505, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[158], &values);

    // Piccolo A#5
    let values: [i32; 7] = [1032545, 1034235, 1034212, 1034227, 44100, 102, 15];
    sample_util::check(&sf.get_sample_headers()[159], &values);

    // Piccolo F#5
    let values: [i32; 7] = [1034267, 1036243, 1036216, 1036235, 44100, 98, 24];
    sample_util::check(&sf.get_sample_headers()[160], &values);

    // Piccolo D5
    let values: [i32; 7] = [1036275, 1038118, 1038086, 1038110, 44100, 94, 29];
    sample_util::check(&sf.get_sample_headers()[161], &values);

    // Piccolo A#4
    let values: [i32; 7] = [1038150, 1039862, 1039825, 1039854, 44100, 90, -44];
    sample_util::check(&sf.get_sample_headers()[162], &values);

    // Bb ff Clar Gb6
    let values: [i32; 7] = [1039894, 1040489, 1040471, 1040481, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[163], &values);

    // Bb ff Clar B4
    let values: [i32; 7] = [1040521, 1042468, 1042429, 1042460, 44100, 89, -28];
    sample_util::check(&sf.get_sample_headers()[164], &values);

    // Bb ff Clar F4
    let values: [i32; 7] = [1042500, 1043716, 1043674, 1043708, 44100, 88, 31];
    sample_util::check(&sf.get_sample_headers()[165], &values);

    // BB ff Clar D4
    let values: [i32; 7] = [1043748, 1045051, 1045003, 1045043, 44100, 85, 13];
    sample_util::check(&sf.get_sample_headers()[166], &values);

    // Bb ff Clar B3
    let values: [i32; 7] = [1045083, 1046330, 1046275, 1046322, 44100, 82, -8];
    sample_util::check(&sf.get_sample_headers()[167], &values);

    // Bb ff Clar F3
    let values: [i32; 7] = [1046362, 1047502, 1047441, 1047494, 44100, 80, 0];
    sample_util::check(&sf.get_sample_headers()[168], &values);

    // Bb ff Clar D3
    let values: [i32; 7] = [1047534, 1049253, 1049167, 1049245, 44100, 73, -31];
    sample_util::check(&sf.get_sample_headers()[169], &values);

    // Bb ff Clar G#2
    let values: [i32; 7] = [1049285, 1051009, 1050867, 1051001, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[170], &values);

    // Bassoon F6
    let values: [i32; 7] = [1051041, 1051533, 1051515, 1051525, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[171], &values);

    // Bassoon E5
    let values: [i32; 7] = [1051565, 1051975, 1051950, 1051967, 44100, 100, 32];
    sample_util::check(&sf.get_sample_headers()[172], &values);

    // Bassoon E3
    let values: [i32; 7] = [1052007, 1053738, 1053658, 1053730, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[173], &values);

    // Bassoon C3
    let values: [i32; 7] = [1053770, 1055737, 1055639, 1055729, 44100, 71, 15];
    sample_util::check(&sf.get_sample_headers()[174], &values);

    // Bassoon G#2
    let values: [i32; 7] = [1055769, 1058558, 1058438, 1058550, 44100, 67, -5];
    sample_util::check(&sf.get_sample_headers()[175], &values);

    // Bassoon E2
    let values: [i32; 7] = [1058590, 1060830, 1060681, 1060822, 44100, 63, -6];
    sample_util::check(&sf.get_sample_headers()[176], &values);

    // Bassoon C2
    let values: [i32; 7] = [1060862, 1062909, 1062724, 1062901, 44100, 59, -12];
    sample_util::check(&sf.get_sample_headers()[177], &values);

    // Bassoon G#1
    let values: [i32; 7] = [1062941, 1065070, 1064840, 1065062, 44100, 55, -20];
    sample_util::check(&sf.get_sample_headers()[178], &values);

    // English Horn F4
    let values: [i32; 7] = [1065102, 1066587, 1066545, 1066579, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[179], &values);

    // English Horn C#4
    let values: [i32; 7] = [1066619, 1067879, 1067829, 1067871, 44100, 84, -3];
    sample_util::check(&sf.get_sample_headers()[180], &values);

    // English Horn A3
    let values: [i32; 7] = [1067911, 1069370, 1069309, 1069362, 44100, 80, 0];
    sample_util::check(&sf.get_sample_headers()[181], &values);

    // English Horn G3
    let values: [i32; 7] = [1069402, 1070847, 1070780, 1070839, 44100, 78, -14];
    sample_util::check(&sf.get_sample_headers()[182], &values);

    // English Horn D#3
    let values: [i32; 7] = [1070879, 1072436, 1072354, 1072428, 44100, 74, -22];
    sample_util::check(&sf.get_sample_headers()[183], &values);

    // English Horn C#3
    let values: [i32; 7] = [1072468, 1073922, 1073831, 1073914, 44100, 72, -23];
    sample_util::check(&sf.get_sample_headers()[184], &values);

    // English Horn B2
    let values: [i32; 7] = [1073954, 1075581, 1075480, 1075573, 44100, 70, -26];
    sample_util::check(&sf.get_sample_headers()[185], &values);

    // English Horn A2
    let values: [i32; 7] = [1075613, 1077784, 1077672, 1077776, 44100, 68, -33];
    sample_util::check(&sf.get_sample_headers()[186], &values);

    // English Horn F2
    let values: [i32; 7] = [1077816, 1079909, 1079758, 1079901, 44100, 63, 18];
    sample_util::check(&sf.get_sample_headers()[187], &values);

    // English Horn F6
    let values: [i32; 7] = [1079941, 1080358, 1080340, 1080350, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[188], &values);

    // SawBass Wave F6
    let values: [i32; 7] = [1080390, 1080548, 1080538, 1080543, 22050, 109, 13];
    sample_util::check(&sf.get_sample_headers()[189], &values);

    // Saw Wave C6
    let values: [i32; 7] = [1080580, 1080596, 1080581, 1080592, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[190], &values);

    // Saw Wave C5
    let values: [i32; 7] = [1080628, 1080655, 1080629, 1080650, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[191], &values);

    // Saw Wave C4
    let values: [i32; 7] = [1080687, 1080735, 1080688, 1080730, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[192], &values);

    // Saw Wave C3
    let values: [i32; 7] = [1080767, 1080857, 1080768, 1080852, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[193], &values);

    // Saw Wave C2
    let values: [i32; 7] = [1080889, 1081063, 1080890, 1081059, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[194], &values);

    // Saw Wave C1
    let values: [i32; 7] = [1081095, 1081438, 1081096, 1081433, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[195], &values);

    // Syn Brass C4
    let values: [i32; 7] = [1081470, 1095179, 1084222, 1095175, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[196], &values);

    // Syn Brass G2
    let values: [i32; 7] = [1095211, 1108765, 1098379, 1108760, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[197], &values);

    // Brass Gb4
    let values: [i32; 7] = [1108797, 1118715, 1111336, 1118711, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[198], &values);

    // Brass C4
    let values: [i32; 7] = [1118747, 1129005, 1121891, 1129001, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[199], &values);

    // Brass C3
    let values: [i32; 7] = [1129037, 1143108, 1133254, 1143104, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[200], &values);

    // Brass C6
    let values: [i32; 7] = [1143140, 1143825, 1143815, 1143821, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[201], &values);

    // Harmon Mute A#4
    let values: [i32; 7] = [1143857, 1146253, 1146219, 1146245, 44100, 92, -33];
    sample_util::check(&sf.get_sample_headers()[202], &values);

    // Harmon Mute C4
    let values: [i32; 7] = [1146285, 1149031, 1148978, 1149023, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[203], &values);

    // Harmon Mute G#3
    let values: [i32; 7] = [1149063, 1151213, 1151149, 1151205, 44100, 79, -4];
    sample_util::check(&sf.get_sample_headers()[204], &values);

    // Harmon Mute E3
    let values: [i32; 7] = [1151245, 1153383, 1153305, 1153375, 44100, 75, -18];
    sample_util::check(&sf.get_sample_headers()[205], &values);

    // Harmon Mute C3
    let values: [i32; 7] = [1153415, 1155989, 1155875, 1155981, 44100, 68, 0];
    sample_util::check(&sf.get_sample_headers()[206], &values);

    // Tuba C4
    let values: [i32; 7] = [1156021, 1156824, 1156777, 1156816, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[207], &values);

    // Tuba A#1
    let values: [i32; 7] = [1156856, 1159898, 1159718, 1159890, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[208], &values);

    // Tuba F#1
    let values: [i32; 7] = [1159930, 1162285, 1162061, 1162277, 44100, 56, 32];
    sample_util::check(&sf.get_sample_headers()[209], &values);

    // Tuba D1
    let values: [i32; 7] = [1162317, 1165062, 1164781, 1165054, 44100, 52, 38];
    sample_util::check(&sf.get_sample_headers()[210], &values);

    // Tuba F#0
    let values: [i32; 7] = [1165094, 1167708, 1167397, 1167700, 44100, 50, 18];
    sample_util::check(&sf.get_sample_headers()[211], &values);

    // Trombone C4
    let values: [i32; 7] = [1167740, 1170374, 1170321, 1170366, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[212], &values);

    // Trombone G3
    let values: [i32; 7] = [1170406, 1172724, 1172657, 1172716, 44100, 78, -14];
    sample_util::check(&sf.get_sample_headers()[213], &values);

    // Trombone D3
    let values: [i32; 7] = [1172756, 1175198, 1175112, 1175190, 44100, 73, -31];
    sample_util::check(&sf.get_sample_headers()[214], &values);

    // Trombone A2
    let values: [i32; 7] = [1175230, 1177782, 1177670, 1177774, 44100, 68, -32];
    sample_util::check(&sf.get_sample_headers()[215], &values);

    // Trombone E2
    let values: [i32; 7] = [1177814, 1178959, 1178814, 1178951, 44100, 64, 44];
    sample_util::check(&sf.get_sample_headers()[216], &values);

    // Soft Trumpet C6
    let values: [i32; 7] = [1178991, 1179176, 1179166, 1179172, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[217], &values);

    // Orch Hit C4
    let values: [i32; 7] = [1179208, 1186785, 1185478, 1185982, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[218], &values);

    // Doo C6
    let values: [i32; 7] = [1186817, 1187851, 1187175, 1187849, 12000, 104, -46];
    sample_util::check(&sf.get_sample_headers()[219], &values);

    // Doo Eb4
    let values: [i32; 7] = [1187883, 1191361, 1189081, 1191359, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[220], &values);

    // Doo B4
    let values: [i32; 7] = [1191393, 1194670, 1192841, 1194667, 12000, 80, 21];
    sample_util::check(&sf.get_sample_headers()[221], &values);

    // Doo G3
    let values: [i32; 7] = [1194702, 1198037, 1195952, 1198034, 12000, 77, 21];
    sample_util::check(&sf.get_sample_headers()[222], &values);

    // Doo Eb3
    let values: [i32; 7] = [1198069, 1201690, 1199609, 1201688, 12000, 73, 21];
    sample_util::check(&sf.get_sample_headers()[223], &values);

    // Doo B2
    let values: [i32; 7] = [1201722, 1204857, 1203010, 1204855, 12000, 69, 21];
    sample_util::check(&sf.get_sample_headers()[224], &values);

    // Syn Vox C4
    let values: [i32; 7] = [1204889, 1211142, 1204892, 1211139, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[225], &values);

    // Syn Vox F#3
    let values: [i32; 7] = [1211174, 1215189, 1211192, 1215186, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[226], &values);

    // Aahs C7
    let values: [i32; 7] = [1215221, 1215719, 1215252, 1215717, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[227], &values);

    // Aahs F4
    let values: [i32; 7] = [1215751, 1228012, 1215764, 1228004, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[228], &values);

    // Aahs C4
    let values: [i32; 7] = [1228044, 1240232, 1228063, 1240224, 44100, 85, 19];
    sample_util::check(&sf.get_sample_headers()[229], &values);

    // Aahs F3
    let values: [i32; 7] = [1240264, 1253064, 1240268, 1253056, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[230], &values);

    // Aahs Db3
    let values: [i32; 7] = [1253096, 1266060, 1253149, 1266052, 44100, 75, 35];
    sample_util::check(&sf.get_sample_headers()[231], &values);

    // SynthStrings G4
    let values: [i32; 7] = [1266092, 1270777, 1266103, 1270773, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[232], &values);

    // SynthStrings G3
    let values: [i32; 7] = [1270809, 1275475, 1270818, 1275470, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[233], &values);

    // SynthStrings G2
    let values: [i32; 7] = [1275507, 1281143, 1275516, 1281139, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[234], &values);

    // Timp Drum A1
    let values: [i32; 7] = [1281175, 1288928, 1287609, 1288921, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[235], &values);

    // Pizz Violin E5
    let values: [i32; 7] = [1288960, 1290315, 1290291, 1290307, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[236], &values);

    // Pizz Violin C4
    let values: [i32; 7] = [1290347, 1294954, 1294904, 1294946, 44100, 84, -3];
    sample_util::check(&sf.get_sample_headers()[237], &values);

    // Pizz Violin E3
    let values: [i32; 7] = [1294986, 1300673, 1300594, 1300665, 44100, 75, 7];
    sample_util::check(&sf.get_sample_headers()[238], &values);

    // SynthStrings D6
    let values: [i32; 7] = [1300705, 1303418, 1300755, 1303414, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[239], &values);

    // Contra Bass F#4
    let values: [i32; 7] = [1303450, 1304107, 1304069, 1304099, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[240], &values);

    // Contra Bass F#2
    let values: [i32; 7] = [1304139, 1309608, 1309362, 1309600, 44100, 54, 0];
    sample_util::check(&sf.get_sample_headers()[241], &values);

    // Contra Bass D2
    let values: [i32; 7] = [1309640, 1313738, 1313430, 1313730, 44100, 50, 0];
    sample_util::check(&sf.get_sample_headers()[242], &values);

    // Arco Cello D4
    let values: [i32; 7] = [1313770, 1314747, 1314688, 1314726, 44100, 86, 24];
    sample_util::check(&sf.get_sample_headers()[243], &values);

    // Arco Cello D2
    let values: [i32; 7] = [1314779, 1316301, 1316104, 1316293, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[244], &values);

    // Arco Cello Gb1
    let values: [i32; 7] = [1316333, 1318846, 1318600, 1318838, 44100, 54, 0];
    sample_util::check(&sf.get_sample_headers()[245], &values);

    // Arco Cello D6
    let values: [i32; 7] = [1318878, 1319137, 1319119, 1319129, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[246], &values);

    // Violin Eb6
    let values: [i32; 7] = [1319169, 1319986, 1319968, 1319978, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[247], &values);

    // Violin Eb5
    let values: [i32; 7] = [1320018, 1321472, 1321445, 1321464, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[248], &values);

    // Violin C5
    let values: [i32; 7] = [1321504, 1322759, 1322729, 1322751, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[249], &values);

    // Violin Gb4
    let values: [i32; 7] = [1322791, 1324122, 1324083, 1324114, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[250], &values);

    // Violin C4
    let values: [i32; 7] = [1324154, 1325497, 1325447, 1325489, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[251], &values);

    // Violin G3
    let values: [i32; 7] = [1325529, 1327055, 1326991, 1327047, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[252], &values);

    // Violin D3
    let values: [i32; 7] = [1327087, 1328680, 1328597, 1328672, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[253], &values);

    // Violin Bb2
    let values: [i32; 7] = [1328712, 1330018, 1329917, 1330011, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[254], &values);

    // SawBassWave F6
    let values: [i32; 7] = [1330050, 1330365, 1330347, 1330357, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[255], &values);

    // SawBassWave F5
    let values: [i32; 7] = [1330397, 1330519, 1330488, 1330511, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[256], &values);

    // SawBassWave Bb3
    let values: [i32; 7] = [1330551, 1332906, 1332747, 1332898, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[257], &values);

    // SawBassWave C3
    let values: [i32; 7] = [1332938, 1335075, 1334933, 1335067, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[258], &values);

    // SawBassWave C2
    let values: [i32; 7] = [1335107, 1337187, 1336911, 1337179, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[259], &values);

    // SawBassWave C1
    let values: [i32; 7] = [1337219, 1339935, 1339392, 1339927, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[260], &values);

    // Slap Bass 2 C6
    let values: [i32; 7] = [1339967, 1340146, 1340127, 1340138, 44100, 107, -21];
    sample_util::check(&sf.get_sample_headers()[261], &values);

    // Slap Bass 2 C4
    let values: [i32; 7] = [1340178, 1341219, 1341164, 1341211, 44100, 82, -11];
    sample_util::check(&sf.get_sample_headers()[262], &values);

    // Slap Bass 2 F1
    let values: [i32; 7] = [1341251, 1343726, 1343467, 1343718, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[263], &values);

    // Slap Bass 2 A0
    let values: [i32; 7] = [1343758, 1346085, 1345627, 1346077, 44100, 43, 1];
    sample_util::check(&sf.get_sample_headers()[264], &values);

    // Slap Bass 1 C6
    let values: [i32; 7] = [1346117, 1346269, 1346250, 1346261, 44100, 107, -21];
    sample_util::check(&sf.get_sample_headers()[265], &values);

    // Slap Bass 1 D4
    let values: [i32; 7] = [1346301, 1346734, 1346684, 1346726, 44100, 84, -3];
    sample_util::check(&sf.get_sample_headers()[266], &values);

    // Slap Bass 1 D1
    let values: [i32; 7] = [1346766, 1351360, 1351012, 1351352, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[267], &values);

    // Slap Bass 1 A0
    let values: [i32; 7] = [1351392, 1359542, 1359080, 1359534, 44100, 43, 19];
    sample_util::check(&sf.get_sample_headers()[268], &values);

    // Fretless Bass C6
    let values: [i32; 7] = [1359574, 1359736, 1359717, 1359728, 44100, 107, -21];
    sample_util::check(&sf.get_sample_headers()[269], &values);

    // Fretless BassDb4
    let values: [i32; 7] = [1359768, 1360623, 1360550, 1360615, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[270], &values);

    // Fretless BassGb1
    let values: [i32; 7] = [1360655, 1366495, 1366271, 1366487, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[271], &values);

    // Fretless Bass C1
    let values: [i32; 7] = [1366527, 1370510, 1370197, 1370502, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[272], &values);

    // Pick Bass D4
    let values: [i32; 7] = [1370542, 1371445, 1371389, 1371437, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[273], &values);

    // Pick Bass D1
    let values: [i32; 7] = [1371477, 1374208, 1373812, 1374200, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[274], &values);

    // Fingered Bass C6
    let values: [i32; 7] = [1374240, 1374354, 1374336, 1374346, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[275], &values);

    // Fingered Bass G3
    let values: [i32; 7] = [1374386, 1374885, 1374831, 1374877, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[276], &values);

    // Fingered Bass C1
    let values: [i32; 7] = [1374917, 1376562, 1376216, 1376554, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[277], &values);

    // Pick Bass C6
    let values: [i32; 7] = [1376594, 1376733, 1376714, 1376725, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[278], &values);

    // GtrHarmonics C6
    let values: [i32; 7] = [1376765, 1376907, 1376883, 1376899, 44100, 105, 38];
    sample_util::check(&sf.get_sample_headers()[279], &values);

    // GtrHarmonics C5
    let values: [i32; 7] = [1376939, 1377279, 1377238, 1377271, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[280], &values);

    // GtrHarmonics Ab3
    let values: [i32; 7] = [1377311, 1378165, 1377990, 1378157, 44100, 76, -1];
    sample_util::check(&sf.get_sample_headers()[281], &values);

    // GtrHarmonics Eb3
    let values: [i32; 7] = [1378197, 1379281, 1379175, 1379273, 44100, 73, -26];
    sample_util::check(&sf.get_sample_headers()[282], &values);

    // GtrHarmonics B2
    let values: [i32; 7] = [1379313, 1382009, 1381628, 1382001, 44100, 69, -15];
    sample_util::check(&sf.get_sample_headers()[283], &values);

    // GtrHarmonics Gb2
    let values: [i32; 7] = [1382041, 1384802, 1384645, 1384794, 44100, 66, -3];
    sample_util::check(&sf.get_sample_headers()[284], &values);

    // Dist Gtr A3
    let values: [i32; 7] = [1384834, 1387252, 1387194, 1387244, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[285], &values);

    // Dist Gtr E3
    let values: [i32; 7] = [1387284, 1389806, 1389731, 1389798, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[286], &values);

    // Dist Gtr B2
    let values: [i32; 7] = [1389838, 1391899, 1391802, 1391891, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[287], &values);

    // Dist Gtr G2
    let values: [i32; 7] = [1391931, 1394235, 1394115, 1394227, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[288], &values);

    // Dist Gtr D2
    let values: [i32; 7] = [1394267, 1396628, 1396470, 1396620, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[289], &values);

    // Dist Gtr A1
    let values: [i32; 7] = [1396660, 1399285, 1399077, 1399277, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[290], &values);

    // Dist Gtr E1
    let values: [i32; 7] = [1399317, 1402968, 1402693, 1402960, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[291], &values);

    // Dist Gtr C6
    let values: [i32; 7] = [1403000, 1403279, 1403261, 1403271, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[292], &values);

    // Dist Gtr E4
    let values: [i32; 7] = [1403311, 1404246, 1404205, 1404238, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[293], &values);

    // OD Gtr C6
    let values: [i32; 7] = [1404278, 1404502, 1404484, 1404494, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[294], &values);

    // OD Gtr E4
    let values: [i32; 7] = [1404534, 1405285, 1405244, 1405277, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[295], &values);

    // OD Gtr A3
    let values: [i32; 7] = [1405317, 1407024, 1406972, 1407016, 44100, 83, -22];
    sample_util::check(&sf.get_sample_headers()[296], &values);

    // OD Gtr E3
    let values: [i32; 7] = [1407056, 1408657, 1408591, 1408649, 44100, 78, -44];
    sample_util::check(&sf.get_sample_headers()[297], &values);

    // OD Gtr B2
    let values: [i32; 7] = [1408689, 1410822, 1410737, 1410814, 44100, 74, 46];
    sample_util::check(&sf.get_sample_headers()[298], &values);

    // OD Gtr G2
    let values: [i32; 7] = [1410854, 1412767, 1412667, 1412759, 44100, 70, -46];
    sample_util::check(&sf.get_sample_headers()[299], &values);

    // OD Gtr D2
    let values: [i32; 7] = [1412799, 1414868, 1414737, 1414860, 44100, 65, -43];
    sample_util::check(&sf.get_sample_headers()[300], &values);

    // OD Gtr A1
    let values: [i32; 7] = [1414900, 1417348, 1417177, 1417340, 44100, 61, 45];
    sample_util::check(&sf.get_sample_headers()[301], &values);

    // OD Gtr E1
    let values: [i32; 7] = [1417380, 1420996, 1420721, 1420988, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[302], &values);

    // Gtr Mute C4
    let values: [i32; 7] = [1421028, 1423005, 1422951, 1422997, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[303], &values);

    // Gtr Mute B2
    let values: [i32; 7] = [1423037, 1425207, 1425110, 1425199, 44100, 71, -3];
    sample_util::check(&sf.get_sample_headers()[304], &values);

    // Gtr Mute G2
    let values: [i32; 7] = [1425239, 1427532, 1427412, 1427524, 44100, 67, -4];
    sample_util::check(&sf.get_sample_headers()[305], &values);

    // Gtr Mute D2
    let values: [i32; 7] = [1427564, 1430050, 1429892, 1430042, 44100, 62, 0];
    sample_util::check(&sf.get_sample_headers()[306], &values);

    // Gtr Mute G1
    let values: [i32; 7] = [1430082, 1432529, 1432297, 1432521, 44100, 55, -4];
    sample_util::check(&sf.get_sample_headers()[307], &values);

    // Strat C4
    let values: [i32; 7] = [1432561, 1435368, 1435314, 1435360, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[308], &values);

    // Strat F#3
    let values: [i32; 7] = [1435400, 1438172, 1438100, 1438164, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[309], &values);

    // Strat C#3
    let values: [i32; 7] = [1438204, 1441476, 1441383, 1441468, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[310], &values);

    // Strat F2
    let values: [i32; 7] = [1441508, 1444813, 1444670, 1444805, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[311], &values);

    // Strat G#1
    let values: [i32; 7] = [1444845, 1448498, 1448264, 1448490, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[312], &values);

    // Jazz Guitar E6
    let values: [i32; 7] = [1448530, 1448924, 1448906, 1448916, 44100, 109, 12];
    sample_util::check(&sf.get_sample_headers()[313], &values);

    // Jazz Guitar E4
    let values: [i32; 7] = [1448956, 1450276, 1450235, 1450268, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[314], &values);

    // Jazz Guitar B4
    let values: [i32; 7] = [1450308, 1451662, 1451610, 1451654, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[315], &values);

    // Jazz Guitar Gb3
    let values: [i32; 7] = [1451694, 1453093, 1453026, 1453085, 44100, 78, -15];
    sample_util::check(&sf.get_sample_headers()[316], &values);

    // Jazz Guitar Eb3
    let values: [i32; 7] = [1453125, 1454535, 1454457, 1454527, 44100, 75, -19];
    sample_util::check(&sf.get_sample_headers()[317], &values);

    // Jazz Guitar Bb2
    let values: [i32; 7] = [1454567, 1455932, 1455831, 1455924, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[318], &values);

    // Jazz Guitar E2
    let values: [i32; 7] = [1455964, 1457313, 1457176, 1457305, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[319], &values);

    // Jazz Guitar C2
    let values: [i32; 7] = [1457345, 1459578, 1458475, 1458637, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[320], &values);

    // Steel AcGtr B2
    let values: [i32; 7] = [1459610, 1461616, 1461493, 1461589, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[321], &values);

    // Steel AcGtr G2
    let values: [i32; 7] = [1461648, 1463778, 1463651, 1463770, 44100, 66, -1];
    sample_util::check(&sf.get_sample_headers()[322], &values);

    // Steel AcGtr D2
    let values: [i32; 7] = [1463810, 1465936, 1465764, 1465928, 44100, 60, -46];
    sample_util::check(&sf.get_sample_headers()[323], &values);

    // Steel AcGtr A1
    let values: [i32; 7] = [1465968, 1472506, 1472280, 1472498, 44100, 56, 48];
    sample_util::check(&sf.get_sample_headers()[324], &values);

    // N Guitar E4
    let values: [i32; 7] = [1472538, 1474716, 1474670, 1474708, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[325], &values);

    // N Guitar C4
    let values: [i32; 7] = [1474748, 1476954, 1476899, 1476946, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[326], &values);

    // N Guitar Ab3
    let values: [i32; 7] = [1476986, 1479207, 1479140, 1479199, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[327], &values);

    // N Guitar E3
    let values: [i32; 7] = [1479239, 1481431, 1481348, 1481423, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[328], &values);

    // N Guitar B2
    let values: [i32; 7] = [1481463, 1483700, 1483598, 1483692, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[329], &values);

    // N Guitar Gb2
    let values: [i32; 7] = [1483732, 1485905, 1485772, 1485897, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[330], &values);

    // N Guitar D2
    let values: [i32; 7] = [1485937, 1488354, 1488186, 1488346, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[331], &values);

    // Harmonica D6
    let values: [i32; 7] = [1488386, 1488624, 1488615, 1488620, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[332], &values);

    // Harmonica D#4
    let values: [i32; 7] = [1488656, 1489586, 1489563, 1489582, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[333], &values);

    // Harmonica C4
    let values: [i32; 7] = [1489618, 1490301, 1490274, 1490297, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[334], &values);

    // Harmonica A3
    let values: [i32; 7] = [1490333, 1491277, 1491246, 1491273, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[335], &values);

    // Harmonica D#3
    let values: [i32; 7] = [1491309, 1492322, 1492280, 1492317, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[336], &values);

    // Harmonica A2
    let values: [i32; 7] = [1492354, 1493479, 1493411, 1493474, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[337], &values);

    // Accordion D6
    let values: [i32; 7] = [1493511, 1493654, 1493645, 1493650, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[338], &values);

    // Accordion D5
    let values: [i32; 7] = [1493686, 1494192, 1494175, 1494188, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[339], &values);

    // Accordion Bb4
    let values: [i32; 7] = [1494224, 1494743, 1494726, 1494739, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[340], &values);

    // Accordion Gb4
    let values: [i32; 7] = [1494775, 1495343, 1495323, 1495339, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[341], &values);

    // Accordion D4
    let values: [i32; 7] = [1495375, 1496000, 1495976, 1495996, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[342], &values);

    // Accordion Bb3
    let values: [i32; 7] = [1496032, 1496865, 1496837, 1496861, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[343], &values);

    // Accordion Gb3
    let values: [i32; 7] = [1496897, 1497723, 1497689, 1497719, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[344], &values);

    // Accordion Bb2
    let values: [i32; 7] = [1497755, 1498678, 1498626, 1498674, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[345], &values);

    // Accordion Gb2
    let values: [i32; 7] = [1498710, 1499761, 1499697, 1499757, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[346], &values);

    // Reed Wave C6
    let values: [i32; 7] = [1499793, 1499832, 1499811, 1499824, 44100, 104, -32];
    sample_util::check(&sf.get_sample_headers()[347], &values);

    // Reed Wave C5
    let values: [i32; 7] = [1499864, 1499930, 1499895, 1499922, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[348], &values);

    // Reed Wave Gb4
    let values: [i32; 7] = [1499962, 1500050, 1500004, 1500042, 44100, 86, 24];
    sample_util::check(&sf.get_sample_headers()[349], &values);

    // Reed Wave C4
    let values: [i32; 7] = [1500082, 1500200, 1500139, 1500192, 44100, 80, 0];
    sample_util::check(&sf.get_sample_headers()[350], &values);

    // Reed Wave Gb3
    let values: [i32; 7] = [1500232, 1500394, 1500311, 1500386, 44100, 74, 0];
    sample_util::check(&sf.get_sample_headers()[351], &values);

    // Reed Wave C3
    let values: [i32; 7] = [1500426, 1500614, 1500500, 1500606, 44100, 68, 0];
    sample_util::check(&sf.get_sample_headers()[352], &values);

    // Reed Wave C2
    let values: [i32; 7] = [1500646, 1500933, 1500713, 1500925, 44100, 56, 0];
    sample_util::check(&sf.get_sample_headers()[353], &values);

    // Church Org F6
    let values: [i32; 7] = [1500965, 1501873, 1501845, 1501865, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[354], &values);

    // Church Org C6
    let values: [i32; 7] = [1501905, 1504597, 1504561, 1504589, 44100, 103, -4];
    sample_util::check(&sf.get_sample_headers()[355], &values);

    // Octave Wave C6
    let values: [i32; 7] = [1504629, 1504657, 1504637, 1504649, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[356], &values);

    // Church Org C4
    let values: [i32; 7] = [1504689, 1507678, 1507586, 1507670, 44100, 84, -3];
    sample_util::check(&sf.get_sample_headers()[357], &values);

    // Church Org C3
    let values: [i32; 7] = [1507710, 1510713, 1510537, 1510705, 44100, 72, -3];
    sample_util::check(&sf.get_sample_headers()[358], &values);

    // Church Org C2
    let values: [i32; 7] = [1510745, 1515665, 1515274, 1515657, 44100, 58, 29];
    sample_util::check(&sf.get_sample_headers()[359], &values);

    // B3LoDistSlow F#5
    let values: [i32; 7] = [1515697, 1516989, 1516955, 1516981, 44100, 104, -32];
    sample_util::check(&sf.get_sample_headers()[360], &values);

    // B3LoDistSlow A4
    let values: [i32; 7] = [1517021, 1527055, 1517997, 1527051, 22050, 95, 35];
    sample_util::check(&sf.get_sample_headers()[361], &values);

    // B3LoDistSlow C4
    let values: [i32; 7] = [1527087, 1535320, 1527545, 1535315, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[362], &values);

    // B3LoDistSlow D#3
    let values: [i32; 7] = [1535352, 1545383, 1535698, 1545378, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[363], &values);

    // B3LoDistSlow F#2
    let values: [i32; 7] = [1545415, 1556140, 1545905, 1556136, 22050, 68, 35];
    sample_util::check(&sf.get_sample_headers()[364], &values);

    // Perc Organ C5
    let values: [i32; 7] = [1556172, 1558106, 1558038, 1558098, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[365], &values);

    // Perc Organ C4
    let values: [i32; 7] = [1558138, 1559618, 1559488, 1559610, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[366], &values);

    // Octave Wave E4
    let values: [i32; 7] = [1559650, 1559735, 1559687, 1559727, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[367], &values);

    // DrawBar Organ C5
    let values: [i32; 7] = [1559767, 1559881, 1559812, 1559873, 44100, 90, 44];
    sample_util::check(&sf.get_sample_headers()[368], &values);

    // DrawBar Organ C3
    let values: [i32; 7] = [1559913, 1560470, 1560250, 1560462, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[369], &values);

    // Dulcimer C6
    let values: [i32; 7] = [1560502, 1561644, 1561622, 1561636, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[370], &values);

    // Dulcimer C4
    let values: [i32; 7] = [1561676, 1568119, 1568055, 1568111, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[371], &values);

    // Steel AcGtr C6
    let values: [i32; 7] = [1568151, 1568399, 1568380, 1568391, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[372], &values);

    // Steel AcGtr A3
    let values: [i32; 7] = [1568431, 1569699, 1569637, 1569691, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[373], &values);

    // Steel AcGtr E3
    let values: [i32; 7] = [1569731, 1571244, 1571164, 1571236, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[374], &values);

    // Xylophone C6
    let values: [i32; 7] = [1571276, 1571632, 1571612, 1571624, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[375], &values);

    // Xylophone C4
    let values: [i32; 7] = [1571664, 1574114, 1574051, 1574106, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[376], &values);

    // Marimba C6
    let values: [i32; 7] = [1574146, 1574730, 1574712, 1574722, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[377], &values);

    // Marimba C3
    let values: [i32; 7] = [1574762, 1579629, 1579537, 1579621, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[378], &values);

    // Marimba C2
    let values: [i32; 7] = [1579661, 1585435, 1585242, 1585427, 44100, 58, -35];
    sample_util::check(&sf.get_sample_headers()[379], &values);

    // Vibes D6
    let values: [i32; 7] = [1585467, 1585898, 1585880, 1585890, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[380], &values);

    // Vibes D4
    let values: [i32; 7] = [1585930, 1590476, 1590428, 1590468, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[381], &values);

    // Vibes E3
    let values: [i32; 7] = [1590508, 1597905, 1597828, 1597897, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[382], &values);

    // Glockenspiel D6
    let values: [i32; 7] = [1597937, 1599038, 1598953, 1599030, 44100, 109, -36];
    sample_util::check(&sf.get_sample_headers()[383], &values);

    // Glockenspiel D5
    let values: [i32; 7] = [1599070, 1607838, 1607379, 1607830, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[384], &values);

    // Celesta C7
    let values: [i32; 7] = [1607870, 1608176, 1608157, 1608168, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[385], &values);

    // Celesta C5
    let values: [i32; 7] = [1608208, 1617780, 1617100, 1617772, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[386], &values);

    // Clavinet C4
    let values: [i32; 7] = [1617812, 1618660, 1618599, 1618652, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[387], &values);

    // Clavinet D6
    let values: [i32; 7] = [1618692, 1618852, 1618834, 1618844, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[388], &values);

    // Clavinet D#3
    let values: [i32; 7] = [1618884, 1620015, 1619918, 1620007, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[389], &values);

    // Clavinet A2
    let values: [i32; 7] = [1620047, 1621592, 1621458, 1621584, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[390], &values);

    // Clavinet D#2
    let values: [i32; 7] = [1621624, 1623207, 1623032, 1623199, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[391], &values);

    // Clavinet A1
    let values: [i32; 7] = [1623239, 1624656, 1624421, 1624648, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[392], &values);

    // Clavinet D#1
    let values: [i32; 7] = [1624688, 1625947, 1625606, 1625939, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[393], &values);

    // Clavinet A0
    let values: [i32; 7] = [1625979, 1627741, 1627279, 1627733, 44100, 43, 19];
    sample_util::check(&sf.get_sample_headers()[394], &values);

    // Harpsichord D6
    let values: [i32; 7] = [1627773, 1628409, 1628391, 1628401, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[395], &values);

    // Harpsichord Ab1
    let values: [i32; 7] = [1628441, 1630601, 1630333, 1630593, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[396], &values);

    // Harpsichord Bb4
    let values: [i32; 7] = [1630633, 1632237, 1632204, 1632229, 44100, 93, -1];
    sample_util::check(&sf.get_sample_headers()[397], &values);

    // Harpsichord A3
    let values: [i32; 7] = [1632269, 1633744, 1633684, 1633736, 44100, 80, -32];
    sample_util::check(&sf.get_sample_headers()[398], &values);

    // Harpsichord C3
    let values: [i32; 7] = [1633776, 1635476, 1635381, 1635468, 44100, 71, -42];
    sample_util::check(&sf.get_sample_headers()[399], &values);

    // Harpsichord E2
    let values: [i32; 7] = [1635508, 1636886, 1636741, 1636878, 44100, 64, 44];
    sample_util::check(&sf.get_sample_headers()[400], &values);

    // DX EP D6
    let values: [i32; 7] = [1636918, 1637561, 1637543, 1637553, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[401], &values);

    // DX EP A4
    let values: [i32; 7] = [1637593, 1639647, 1639607, 1639639, 44100, 89, 27];
    sample_util::check(&sf.get_sample_headers()[402], &values);

    // DX EP C4
    let values: [i32; 7] = [1639679, 1641248, 1641195, 1641240, 44100, 83, 16];
    sample_util::check(&sf.get_sample_headers()[403], &values);

    // DX EP Gb3
    let values: [i32; 7] = [1641280, 1643448, 1643377, 1643440, 44100, 77, -1];
    sample_util::check(&sf.get_sample_headers()[404], &values);

    // DX EP A2
    let values: [i32; 7] = [1643480, 1645776, 1645663, 1645768, 44100, 68, -17];
    sample_util::check(&sf.get_sample_headers()[405], &values);

    // DX EP C2
    let values: [i32; 7] = [1645808, 1648610, 1648411, 1648602, 44100, 58, 19];
    sample_util::check(&sf.get_sample_headers()[406], &values);

    // EP1 C6
    let values: [i32; 7] = [1648642, 1648806, 1648788, 1648798, 44100, 109, 12];
    sample_util::check(&sf.get_sample_headers()[407], &values);

    // EP1 C4
    let values: [i32; 7] = [1648838, 1649513, 1649463, 1649505, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[408], &values);

    // EP1 G3
    let values: [i32; 7] = [1649545, 1650415, 1650356, 1650407, 44100, 81, 31];
    sample_util::check(&sf.get_sample_headers()[409], &values);

    // EP1 E3
    let values: [i32; 7] = [1650447, 1651314, 1651246, 1651306, 44100, 78, 13];
    sample_util::check(&sf.get_sample_headers()[410], &values);

    // EP1 C3
    let values: [i32; 7] = [1651346, 1652231, 1652147, 1652223, 44100, 74, 21];
    sample_util::check(&sf.get_sample_headers()[411], &values);

    // EP1 Ab2
    let values: [i32; 7] = [1652263, 1654161, 1653961, 1654153, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[412], &values);

    // EP1 2 C6
    let values: [i32; 7] = [1654193, 1654522, 1654502, 1654514, 44100, 106, 29];
    sample_util::check(&sf.get_sample_headers()[413], &values);

    // EP1 2 C4
    let values: [i32; 7] = [1654554, 1655858, 1655802, 1655850, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[414], &values);

    // EP1 2 Gb3
    let values: [i32; 7] = [1655890, 1657147, 1657071, 1657139, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[415], &values);

    // EP1 2 C3
    let values: [i32; 7] = [1657179, 1658425, 1658322, 1658417, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[416], &values);

    // EP1 2 C2
    let values: [i32; 7] = [1658457, 1659857, 1659688, 1659849, 44100, 61, 24];
    sample_util::check(&sf.get_sample_headers()[417], &values);

    // EP1 2 C1
    let values: [i32; 7] = [1659889, 1662117, 1661742, 1662109, 44100, 47, 50];
    sample_util::check(&sf.get_sample_headers()[418], &values);

    // CP70 D#6
    let values: [i32; 7] = [1662149, 1662407, 1662389, 1662399, 44100, 109, 13];
    sample_util::check(&sf.get_sample_headers()[419], &values);

    // CP70 D#5
    let values: [i32; 7] = [1662439, 1662977, 1662948, 1662969, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[420], &values);

    // CP70 A4
    let values: [i32; 7] = [1663009, 1663592, 1663553, 1663584, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[421], &values);

    // CP70 C4
    let values: [i32; 7] = [1663624, 1664154, 1664095, 1664146, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[422], &values);

    // CP70 C3
    let values: [i32; 7] = [1664186, 1664700, 1664591, 1664692, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[423], &values);

    // CP70 D#2
    let values: [i32; 7] = [1664732, 1665396, 1665230, 1665388, 44100, 60, 0];
    sample_util::check(&sf.get_sample_headers()[424], &values);

    // AltoLoF
    let values: [i32; 7] = [1665428, 1676621, 1672105, 1676479, 22050, 56, 3];
    sample_util::check(&sf.get_sample_headers()[425], &values);

    // AltoHiB
    let values: [i32; 7] = [1676653, 1688522, 1684122, 1688522, 22500, 74, 1];
    sample_util::check(&sf.get_sample_headers()[426], &values);

    // AltoAlt
    let values: [i32; 7] = [1688554, 1696252, 1694827, 1696159, 22500, 82, -28];
    sample_util::check(&sf.get_sample_headers()[427], &values);

    // AltoLoA
    let values: [i32; 7] = [1696284, 1700442, 1700135, 1700441, 22500, 50, 0];
    sample_util::check(&sf.get_sample_headers()[428], &values);

    // AltoG
    let values: [i32; 7] = [1700474, 1712019, 1707354, 1712008, 22500, 58, 11];
    sample_util::check(&sf.get_sample_headers()[429], &values);

    // AltoHiF
    let values: [i32; 7] = [1712051, 1733484, 1729048, 1733473, 22500, 80, -21];
    sample_util::check(&sf.get_sample_headers()[430], &values);

    // TenorLoE
    let values: [i32; 7] = [1733516, 1735742, 1735601, 1735734, 44100, 64, -7];
    sample_util::check(&sf.get_sample_headers()[431], &values);

    // TenorG#
    let values: [i32; 7] = [1735774, 1737333, 1737213, 1737325, 44100, 67, -4];
    sample_util::check(&sf.get_sample_headers()[432], &values);

    // TenorD
    let values: [i32; 7] = [1737365, 1738524, 1738441, 1738516, 44100, 74, 0];
    sample_util::check(&sf.get_sample_headers()[433], &values);

    // TenorHiG
    let values: [i32; 7] = [1738556, 1739914, 1739843, 1739906, 44100, 77, -1];
    sample_util::check(&sf.get_sample_headers()[434], &values);

    // TenorAlt
    let values: [i32; 7] = [1739946, 1741686, 1741636, 1741678, 44100, 84, -3];
    sample_util::check(&sf.get_sample_headers()[435], &values);

    // TenorLoB
    let values: [i32; 7] = [1741718, 1743870, 1743694, 1743862, 44100, 60, -3];
    sample_util::check(&sf.get_sample_headers()[436], &values);

    // TenorHiC
    let values: [i32; 7] = [1743902, 1745449, 1745406, 1745441, 44100, 87, -18];
    sample_util::check(&sf.get_sample_headers()[437], &values);

    // BariAb3
    let values: [i32; 7] = [1745481, 1754938, 1753416, 1754934, 22500, 44, 25];
    sample_util::check(&sf.get_sample_headers()[438], &values);

    // SopHiF
    let values: [i32; 7] = [1754970, 1771196, 1768629, 1770955, 22500, 87, -24];
    sample_util::check(&sf.get_sample_headers()[439], &values);

    // SopHiD
    let values: [i32; 7] = [1771228, 1787167, 1782983, 1786966, 22500, 84, -27];
    sample_util::check(&sf.get_sample_headers()[440], &values);

    // SopHiB
    let values: [i32; 7] = [1787199, 1809611, 1805673, 1809608, 22500, 81, -30];
    sample_util::check(&sf.get_sample_headers()[441], &values);

    // SopHiG#
    let values: [i32; 7] = [1809643, 1831266, 1827365, 1831255, 22500, 78, -13];
    sample_util::check(&sf.get_sample_headers()[442], &values);

    // SopF
    let values: [i32; 7] = [1831298, 1856268, 1851819, 1856134, 22500, 75, -35];
    sample_util::check(&sf.get_sample_headers()[443], &values);

    // SopD
    let values: [i32; 7] = [1856300, 1879363, 1874978, 1879039, 22500, 72, -27];
    sample_util::check(&sf.get_sample_headers()[444], &values);

    // SopB
    let values: [i32; 7] = [1879395, 1904039, 1897522, 1901697, 22500, 69, -25];
    sample_util::check(&sf.get_sample_headers()[445], &values);

    // SopG
    let values: [i32; 7] = [1904071, 1933239, 1928903, 1933096, 22500, 65, -23];
    sample_util::check(&sf.get_sample_headers()[446], &values);

    // SopLowE
    let values: [i32; 7] = [1933271, 1965973, 1961405, 1965727, 22500, 62, -14];
    sample_util::check(&sf.get_sample_headers()[447], &values);

    // SopLowB
    let values: [i32; 7] = [1966005, 1991196, 1981582, 1991171, 22500, 57, -26];
    sample_util::check(&sf.get_sample_headers()[448], &values);

    // AltoC
    let values: [i32; 7] = [1991228, 2012114, 2007327, 2012035, 22500, 63, -21];
    sample_util::check(&sf.get_sample_headers()[449], &values);

    // AltoD
    let values: [i32; 7] = [2012146, 2021699, 2016975, 2021691, 22500, 65, -16];
    sample_util::check(&sf.get_sample_headers()[450], &values);

    // AltoLoC
    let values: [i32; 7] = [2021731, 2039536, 2034878, 2039511, 22500, 51, 8];
    sample_util::check(&sf.get_sample_headers()[451], &values);

    // AltoF
    let values: [i32; 7] = [2039568, 2054197, 2049738, 2054196, 22500, 68, -19];
    sample_util::check(&sf.get_sample_headers()[452], &values);

    // AltoHiA
    let values: [i32; 7] = [2054229, 2064430, 2059759, 2064406, 22500, 72, -14];
    sample_util::check(&sf.get_sample_headers()[453], &values);

    // AltoHiD
    let values: [i32; 7] = [2064462, 2075496, 2070777, 2075481, 22500, 77, -10];
    sample_util::check(&sf.get_sample_headers()[454], &values);

    // AltoLoD#
    let values: [i32; 7] = [2075528, 2088331, 2083522, 2088306, 22500, 54, 18];
    sample_util::check(&sf.get_sample_headers()[455], &values);

    // AltoA
    let values: [i32; 7] = [2088363, 2099981, 2094864, 2099959, 22500, 60, 3];
    sample_util::check(&sf.get_sample_headers()[456], &values);

    // BariC4
    let values: [i32; 7] = [2100013, 2109665, 2108451, 2109660, 22500, 48, 13];
    sample_util::check(&sf.get_sample_headers()[457], &values);

    // BariE4
    let values: [i32; 7] = [2109697, 2119785, 2119097, 2119781, 22500, 52, 6];
    sample_util::check(&sf.get_sample_headers()[458], &values);

    // BariAb4
    let values: [i32; 7] = [2119817, 2131435, 2130780, 2131430, 22500, 56, 12];
    sample_util::check(&sf.get_sample_headers()[459], &values);

    // BariC5
    let values: [i32; 7] = [2131467, 2140913, 2140477, 2140908, 22500, 60, 3];
    sample_util::check(&sf.get_sample_headers()[460], &values);

    // BariE5
    let values: [i32; 7] = [2140945, 2152016, 2151602, 2152012, 22500, 64, 6];
    sample_util::check(&sf.get_sample_headers()[461], &values);

    // BariAb5
    let values: [i32; 7] = [2152048, 2163142, 2162975, 2163138, 22500, 68, 3];
    sample_util::check(&sf.get_sample_headers()[462], &values);

    // BariC6
    let values: [i32; 7] = [2163174, 2169525, 2169480, 2169523, 22500, 72, 3];
    sample_util::check(&sf.get_sample_headers()[463], &values);

    // Tuba A#2
    let values: [i32; 7] = [2169557, 2172053, 2171962, 2172048, 22050, 60, 38];
    sample_util::check(&sf.get_sample_headers()[464], &values);

    // Tuba F#2
    let values: [i32; 7] = [2172085, 2174184, 2174071, 2174179, 22050, 56, 32];
    sample_util::check(&sf.get_sample_headers()[465], &values);

    // Tuba D2
    let values: [i32; 7] = [2174216, 2176630, 2176489, 2176626, 22050, 52, 38];
    sample_util::check(&sf.get_sample_headers()[466], &values);

    // Tuba F#3
    let values: [i32; 7] = [2176662, 2177969, 2177813, 2177965, 22050, 50, 18];
    sample_util::check(&sf.get_sample_headers()[467], &values);

    // Trombone C5
    let values: [i32; 7] = [2178001, 2179582, 2179551, 2179578, 22050, 80, 32];
    sample_util::check(&sf.get_sample_headers()[468], &values);

    // Trombone G4
    let values: [i32; 7] = [2179614, 2181028, 2180988, 2181024, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[469], &values);

    // FRHORN00
    let values: [i32; 7] = [2181060, 2195645, 2190284, 2195642, 22050, 58, 17];
    sample_util::check(&sf.get_sample_headers()[470], &values);

    // FRHORN01
    let values: [i32; 7] = [2195677, 2208863, 2202979, 2208855, 22050, 65, 29];
    sample_util::check(&sf.get_sample_headers()[471], &values);

    // FRHORN02
    let values: [i32; 7] = [2208895, 2223299, 2217553, 2223296, 22050, 68, 26];
    sample_util::check(&sf.get_sample_headers()[472], &values);

    // FRHORN03
    let values: [i32; 7] = [2223331, 2239107, 2232922, 2239105, 22050, 75, 32];
    sample_util::check(&sf.get_sample_headers()[473], &values);

    // A#2 FrenchHorn1
    let values: [i32; 7] = [2239139, 2258022, 2248397, 2257990, 22050, 56, 24];
    sample_util::check(&sf.get_sample_headers()[474], &values);

    // G3 FrenchHorn1
    let values: [i32; 7] = [2258054, 2275244, 2264615, 2275193, 22050, 65, 16];
    sample_util::check(&sf.get_sample_headers()[475], &values);

    // C4 FrenchHorn1
    let values: [i32; 7] = [2275276, 2291183, 2281306, 2291138, 22050, 70, 18];
    sample_util::check(&sf.get_sample_headers()[476], &values);

    // G4 FrenchHorn1
    let values: [i32; 7] = [2291215, 2306941, 2298023, 2306909, 22050, 77, 18];
    sample_util::check(&sf.get_sample_headers()[477], &values);

    // C5 FrenchHorn1
    let values: [i32; 7] = [2306973, 2324357, 2313440, 2324310, 22050, 82, 18];
    sample_util::check(&sf.get_sample_headers()[478], &values);

    // French Horns Eb2
    let values: [i32; 7] = [2324389, 2327387, 2327303, 2327383, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[479], &values);

    // French Horns G2
    let values: [i32; 7] = [2327419, 2330812, 2330744, 2330807, 22050, 65, -2];
    sample_util::check(&sf.get_sample_headers()[480], &values);

    // French Horns B2
    let values: [i32; 7] = [2330844, 2334040, 2333985, 2334035, 22050, 69, -2];
    sample_util::check(&sf.get_sample_headers()[481], &values);

    // French Horns Eb3
    let values: [i32; 7] = [2334072, 2336026, 2335982, 2336022, 22050, 73, -10];
    sample_util::check(&sf.get_sample_headers()[482], &values);

    // French Horns B3
    let values: [i32; 7] = [2336058, 2338425, 2338396, 2338421, 22050, 60, 0];
    sample_util::check(&sf.get_sample_headers()[483], &values);

    // French Horns C6
    let values: [i32; 7] = [2338457, 2338660, 2338650, 2338656, 22050, 107, -22];
    sample_util::check(&sf.get_sample_headers()[484], &values);

    // Aahs C6
    let values: [i32; 7] = [2338692, 2340059, 2338696, 2340057, 12000, 104, -46];
    sample_util::check(&sf.get_sample_headers()[485], &values);

    // Aahs F5
    let values: [i32; 7] = [2340091, 2344192, 2340091, 2344189, 12000, 85, -46];
    sample_util::check(&sf.get_sample_headers()[486], &values);

    // Aahs C5
    let values: [i32; 7] = [2344224, 2348166, 2344224, 2348163, 12000, 82, 21];
    sample_util::check(&sf.get_sample_headers()[487], &values);

    // Aahs A3
    let values: [i32; 7] = [2348198, 2353212, 2348198, 2353209, 12000, 79, 21];
    sample_util::check(&sf.get_sample_headers()[488], &values);

    // Aahs F6
    let values: [i32; 7] = [2353244, 2357597, 2353244, 2357593, 12000, 75, 21];
    sample_util::check(&sf.get_sample_headers()[489], &values);

    // Aahs Db4
    let values: [i32; 7] = [2357629, 2362045, 2357629, 2362042, 12000, 71, 21];
    sample_util::check(&sf.get_sample_headers()[490], &values);

    // Aahs Ab2
    let values: [i32; 7] = [2362077, 2366482, 2362077, 2366479, 12000, 66, 21];
    sample_util::check(&sf.get_sample_headers()[491], &values);

    // Aahs Eb2
    let values: [i32; 7] = [2366514, 2371614, 2366514, 2371611, 12000, 59, -46];
    sample_util::check(&sf.get_sample_headers()[492], &values);

    // Choir-D#21
    let values: [i32; 7] = [2371646, 2384490, 2371650, 2384485, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[493], &values);

    // Choir-A21
    let values: [i32; 7] = [2384522, 2403187, 2384526, 2403182, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[494], &values);

    // Choir-D#31
    let values: [i32; 7] = [2403219, 2423931, 2403223, 2423926, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[495], &values);

    // Choir-A31
    let values: [i32; 7] = [2423963, 2440212, 2423967, 2440206, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[496], &values);

    // Choir-F#41
    let values: [i32; 7] = [2440244, 2460185, 2440248, 2460180, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[497], &values);

    // Choir-D#51
    let values: [i32; 7] = [2460217, 2475401, 2460220, 2475396, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[498], &values);

    // Choir-A51
    let values: [i32; 7] = [2475433, 2484803, 2475435, 2484801, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[499], &values);

    // Synth Strings 2-C2
    let values: [i32; 7] = [2484835, 2494584, 2484837, 2494579, 22500, 60, 0];
    sample_util::check(&sf.get_sample_headers()[500], &values);

    // Synth Strings 2-C3
    let values: [i32; 7] = [2494616, 2504730, 2494618, 2504725, 22500, 60, 0];
    sample_util::check(&sf.get_sample_headers()[501], &values);

    // Synth Strings 2-C4
    let values: [i32; 7] = [2504762, 2517848, 2504764, 2517843, 22500, 60, 0];
    sample_util::check(&sf.get_sample_headers()[502], &values);

    // Synth Strings 2-C5
    let values: [i32; 7] = [2517880, 2529581, 2517882, 2529576, 22500, 60, 0];
    sample_util::check(&sf.get_sample_headers()[503], &values);

    // ensstringsg2
    let values: [i32; 7] = [2529613, 2555806, 2542165, 2555804, 12000, 43, -3];
    sample_util::check(&sf.get_sample_headers()[504], &values);

    // ensstringsc3
    let values: [i32; 7] = [2555838, 2579503, 2564633, 2579502, 12000, 48, -3];
    sample_util::check(&sf.get_sample_headers()[505], &values);

    // ensstringsc4
    let values: [i32; 7] = [2579535, 2600279, 2585607, 2600279, 12000, 60, -5];
    sample_util::check(&sf.get_sample_headers()[506], &values);

    // ensstringsg4
    let values: [i32; 7] = [2600311, 2625494, 2606853, 2625493, 12000, 67, -5];
    sample_util::check(&sf.get_sample_headers()[507], &values);

    // ensstringsc5
    let values: [i32; 7] = [2625526, 2648713, 2634213, 2648712, 12000, 72, -7];
    sample_util::check(&sf.get_sample_headers()[508], &values);

    // ensstringsg5
    let values: [i32; 7] = [2648745, 2673394, 2656663, 2673393, 12000, 79, -6];
    sample_util::check(&sf.get_sample_headers()[509], &values);

    // ensstringsc6
    let values: [i32; 7] = [2673426, 2686612, 2677378, 2686611, 12000, 84, -4];
    sample_util::check(&sf.get_sample_headers()[510], &values);

    // ensstringsg6
    let values: [i32; 7] = [2686644, 2712833, 2699598, 2712833, 12000, 91, -5];
    sample_util::check(&sf.get_sample_headers()[511], &values);

    // ensstringsc7
    let values: [i32; 7] = [2712865, 2727420, 2720001, 2727420, 12000, 96, -5];
    sample_util::check(&sf.get_sample_headers()[512], &values);

    // Str1C4 R
    let values: [i32; 7] = [2727452, 2759140, 2741576, 2756808, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[513], &values);

    // Str1C5 R
    let values: [i32; 7] = [2759172, 2781722, 2768395, 2779245, 12000, 72, 0];
    sample_util::check(&sf.get_sample_headers()[514], &values);

    // Str1C6 R
    let values: [i32; 7] = [2781754, 2803406, 2788890, 2801499, 12000, 84, 0];
    sample_util::check(&sf.get_sample_headers()[515], &values);

    // Str1C4 L
    let values: [i32; 7] = [2803438, 2835126, 2809376, 2833342, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[516], &values);

    // Str1C5 L
    let values: [i32; 7] = [2835158, 2857708, 2842936, 2857350, 12000, 72, 0];
    sample_util::check(&sf.get_sample_headers()[517], &values);

    // Str1C6 L
    let values: [i32; 7] = [2857740, 2879392, 2867448, 2879352, 12000, 84, 0];
    sample_util::check(&sf.get_sample_headers()[518], &values);

    // SynthStringsC4
    let values: [i32; 7] = [2879424, 2882136, 2879770, 2882135, 12000, 60, 0];
    sample_util::check(&sf.get_sample_headers()[519], &values);
}
