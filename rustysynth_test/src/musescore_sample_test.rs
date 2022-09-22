#![allow(unused_imports)]

use std::fs::File;
use std::path::PathBuf;
use rustysynth::SoundFont;

use crate::sample_util;

#[test]
fn samples()
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.push("GeneralUser GS MuseScore v1.442.sf2");
    let mut file = File::open(&path).unwrap();
    let sf = SoundFont::new(&mut file).unwrap();

    // GUTest-version144
    let values: [i32; 7] = [0, 28607, 8, 28599, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[0], &values);

    // Overdrive Guitar-G4
    let values: [i32; 7] = [28639, 63135, 62658, 62998, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[1], &values);

    // Overdrive Guitar-G3
    let values: [i32; 7] = [63167, 80191, 79699, 80150, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[2], &values);

    // Overdrive Guitar-G2
    let values: [i32; 7] = [80223, 97247, 96527, 97198, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[3], &values);

    // Overdrive Guitar-F4
    let values: [i32; 7] = [97279, 146879, 146496, 146687, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[4], &values);

    // Overdrive Guitar-E5
    let values: [i32; 7] = [146911, 183819, 183710, 183811, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[5], &values);

    // Overdrive Guitar-E3
    let values: [i32; 7] = [183851, 198635, 198196, 198596, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[6], &values);

    // Overdrive Guitar-E2
    let values: [i32; 7] = [198667, 216907, 216577, 216844, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[7], &values);

    // Overdrive Guitar-D6
    let values: [i32; 7] = [216939, 232107, 231437, 231925, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[8], &values);

    // Overdrive Guitar-G5
    let values: [i32; 7] = [232139, 240075, 239206, 239829, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[9], &values);

    // Overdrive Guitar-D4
    let values: [i32; 7] = [240107, 265131, 264314, 264994, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[10], &values);

    // Overdrive Guitar-D3
    let values: [i32; 7] = [265163, 281323, 280463, 281212, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[11], &values);

    // Overdrive Guitar-D2
    let values: [i32; 7] = [281355, 303293, 302984, 303283, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[12], &values);

    // Overdrive Guitar-C6
    let values: [i32; 7] = [303325, 318524, 317801, 318415, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[13], &values);

    // Overdrive Guitar-C5
    let values: [i32; 7] = [318556, 332700, 332133, 332472, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[14], &values);

    // Overdrive Guitar-C4
    let values: [i32; 7] = [332732, 348092, 347107, 347868, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[15], &values);

    // Overdrive Guitar-C3
    let values: [i32; 7] = [348124, 378076, 377641, 377977, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[16], &values);

    // Overdrive Guitar-A5
    let values: [i32; 7] = [378108, 390460, 389747, 390252, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[17], &values);

    // Overdrive Guitar-A4
    let values: [i32; 7] = [390492, 405019, 404686, 404938, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[18], &values);

    // Overdrive Guitar-A3
    let values: [i32; 7] = [405051, 426843, 426065, 426667, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[19], &values);

    // Overdrive Guitar-A2
    let values: [i32; 7] = [426875, 441578, 440118, 441517, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[20], &values);

    // Overdrive Guitar-D5
    let values: [i32; 7] = [441610, 473098, 472206, 472925, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[21], &values);

    // Alto Sax-G4
    let values: [i32; 7] = [473130, 491192, 490916, 491139, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[22], &values);

    // Alto Sax-G3
    let values: [i32; 7] = [491224, 512826, 508044, 512778, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[23], &values);

    // Alto Sax-F#5
    let values: [i32; 7] = [512858, 531961, 530699, 531807, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[24], &values);

    // Alto Sax-F6
    let values: [i32; 7] = [531993, 551863, 550830, 551682, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[25], &values);

    // Alto Sax-F3
    let values: [i32; 7] = [551895, 573713, 573357, 573610, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[26], &values);

    // Alto Sax-E5
    let values: [i32; 7] = [573745, 602132, 601261, 602049, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[27], &values);

    // Alto Sax-E4
    let values: [i32; 7] = [602164, 619915, 619297, 619429, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[28], &values);

    // Alto Sax-D4
    let values: [i32; 7] = [619947, 651164, 634503, 651142, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[29], &values);

    // Alto Sax-C5
    let values: [i32; 7] = [651196, 664120, 663836, 664087, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[30], &values);

    // Alto Sax-A#3
    let values: [i32; 7] = [664152, 677894, 676437, 677574, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[31], &values);

    // Alto Sax-A5
    let values: [i32; 7] = [677926, 694261, 693823, 693922, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[32], &values);

    // Alto Sax-A4
    let values: [i32; 7] = [694293, 708991, 708430, 708827, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[33], &values);

    // Alto Sax-G#5
    let values: [i32; 7] = [709023, 723652, 722888, 723612, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[34], &values);

    // Taiko Drum
    let values: [i32; 7] = [723684, 735537, 733155, 735529, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[35], &values);

    // English Horn-F#4
    let values: [i32; 7] = [735569, 756729, 750512, 756695, 27174, 66, 0];
    sample_util::check(&sf.sample_headers[36], &values);

    // English Horn-F#3
    let values: [i32; 7] = [756761, 782265, 775911, 782233, 27174, 54, 0];
    sample_util::check(&sf.sample_headers[37], &values);

    // English Horn-C#4
    let values: [i32; 7] = [782297, 810217, 804661, 810185, 27174, 61, 0];
    sample_util::check(&sf.sample_headers[38], &values);

    // English Horn-C#3
    let values: [i32; 7] = [810249, 830869, 824996, 830837, 27174, 49, 0];
    sample_util::check(&sf.sample_headers[39], &values);

    // English Horn-A#3
    let values: [i32; 7] = [830901, 851916, 845659, 851884, 27174, 58, 0];
    sample_util::check(&sf.sample_headers[40], &values);

    // English Horn-A#2
    let values: [i32; 7] = [851948, 863415, 857593, 863383, 27174, 46, 0];
    sample_util::check(&sf.sample_headers[41], &values);

    // English Horn-G#4
    let values: [i32; 7] = [863447, 885217, 878530, 885185, 27174, 68, 0];
    sample_util::check(&sf.sample_headers[42], &values);

    // Violin-F#5
    let values: [i32; 7] = [885249, 922513, 908141, 922509, 27174, 78, 0];
    sample_util::check(&sf.sample_headers[43], &values);

    // Violin-F#4
    let values: [i32; 7] = [922545, 958866, 942833, 957812, 27174, 66, 0];
    sample_util::check(&sf.sample_headers[44], &values);

    // Violin-F5
    let values: [i32; 7] = [958898, 994461, 981506, 994456, 27174, 77, 0];
    sample_util::check(&sf.sample_headers[45], &values);

    // Violin-D#6
    let values: [i32; 7] = [994493, 1028536, 1015071, 1028532, 27174, 87, 0];
    sample_util::check(&sf.sample_headers[46], &values);

    // Violin-D#4
    let values: [i32; 7] = [1028568, 1064935, 1049986, 1064913, 27174, 63, 0];
    sample_util::check(&sf.sample_headers[47], &values);

    // Violin-C#5
    let values: [i32; 7] = [1064967, 1100302, 1079776, 1094775, 27174, 73, 0];
    sample_util::check(&sf.sample_headers[48], &values);

    // Violin-B5
    let values: [i32; 7] = [1100334, 1137869, 1124182, 1137866, 27174, 83, 0];
    sample_util::check(&sf.sample_headers[49], &values);

    // Violin-B3
    let values: [i32; 7] = [1137901, 1177445, 1161823, 1177443, 27174, 59, 0];
    sample_util::check(&sf.sample_headers[50], &values);

    // Violin-A#4
    let values: [i32; 7] = [1177477, 1215606, 1201012, 1215600, 27174, 70, 0];
    sample_util::check(&sf.sample_headers[51], &values);

    // Violin-A3
    let values: [i32; 7] = [1215638, 1252236, 1238579, 1252234, 27174, 56, 0];
    sample_util::check(&sf.sample_headers[52], &values);

    // Violin-F#6
    let values: [i32; 7] = [1252268, 1292941, 1280896, 1292937, 27174, 90, 0];
    sample_util::check(&sf.sample_headers[53], &values);

    // Celeste-D#4
    let values: [i32; 7] = [1292973, 1301086, 1300867, 1301080, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[54], &values);

    // Celeste-F#5
    let values: [i32; 7] = [1301118, 1308829, 1308599, 1308823, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[55], &values);

    // Celeste-D#7-delay
    let values: [i32; 7] = [1308861, 1320071, 1319692, 1320065, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[56], &values);

    // Celeste-F#5-delay
    let values: [i32; 7] = [1320103, 1329464, 1329234, 1329458, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[57], &values);

    // GUTest-velscale
    let values: [i32; 7] = [1329496, 1339987, 1329504, 1339979, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[58], &values);

    // GUTest-testing
    let values: [i32; 7] = [1340019, 1344060, 1340027, 1344052, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[59], &values);

    // GUTest-supported
    let values: [i32; 7] = [1344092, 1348382, 1344100, 1348374, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[60], &values);

    // GUTest-reverb
    let values: [i32; 7] = [1348414, 1352920, 1348422, 1352912, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[61], &values);

    // GUTest-offset
    let values: [i32; 7] = [1352952, 1360937, 1352960, 1360929, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[62], &values);

    // GUTest-notsupported
    let values: [i32; 7] = [1360969, 1367509, 1360977, 1367501, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[63], &values);

    // GUTest-love
    let values: [i32; 7] = [1367541, 1375334, 1367549, 1375326, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[64], &values);

    // GUTest-chorus
    let values: [i32; 7] = [1375366, 1379272, 1375374, 1379264, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[65], &values);

    // Harpsichord-Bb4
    let values: [i32; 7] = [1379304, 1380106, 1380073, 1380098, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[66], &values);

    // Harpsichord-A3
    let values: [i32; 7] = [1380138, 1380841, 1380781, 1380833, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[67], &values);

    // Harpsichord-C3
    let values: [i32; 7] = [1380873, 1381791, 1381696, 1381783, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[68], &values);

    // Harpsichord-E2
    let values: [i32; 7] = [1381823, 1382476, 1382331, 1382468, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[69], &values);

    // Harpsichord-Ab1
    let values: [i32; 7] = [1382508, 1383591, 1383323, 1383583, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[70], &values);

    // French Horn D#6    
    let values: [i32; 7] = [1383623, 1384048, 1384033, 1384043, 24891, 60, 0];
    sample_util::check(&sf.sample_headers[71], &values);

    // French Horn D#4    
    let values: [i32; 7] = [1384080, 1389467, 1389409, 1389462, 32984, 60, 0];
    sample_util::check(&sf.sample_headers[72], &values);

    // FrenchHrn A#3-43   
    let values: [i32; 7] = [1389499, 1392876, 1392801, 1392871, 32632, 60, 0];
    sample_util::check(&sf.sample_headers[73], &values);

    // French Horn F3     
    let values: [i32; 7] = [1392908, 1397959, 1397861, 1397954, 32479, 60, 0];
    sample_util::check(&sf.sample_headers[74], &values);

    // French Horn C3     
    let values: [i32; 7] = [1397991, 1406330, 1406203, 1406325, 31920, 60, 0];
    sample_util::check(&sf.sample_headers[75], &values);

    // French Horn F2     
    let values: [i32; 7] = [1406362, 1410815, 1410633, 1410810, 30908, 60, 0];
    sample_util::check(&sf.sample_headers[76], &values);

    // French Horn D2     
    let values: [i32; 7] = [1410847, 1415099, 1414890, 1415094, 29954, 60, 0];
    sample_util::check(&sf.sample_headers[77], &values);

    // Fr Horn mf D#6     
    let values: [i32; 7] = [1415131, 1415493, 1415478, 1415488, 24891, 60, 0];
    sample_util::check(&sf.sample_headers[78], &values);

    // Fr Horn mf D#4     
    let values: [i32; 7] = [1415525, 1420379, 1420330, 1420374, 27385, 60, 0];
    sample_util::check(&sf.sample_headers[79], &values);

    // FrHorn mf A#3-41   
    let values: [i32; 7] = [1420411, 1426368, 1426304, 1426363, 27507, 60, 0];
    sample_util::check(&sf.sample_headers[80], &values);

    // FrHornmf F3 -44    
    let values: [i32; 7] = [1426400, 1430277, 1430193, 1430272, 27592, 60, 0];
    sample_util::check(&sf.sample_headers[81], &values);

    // Fr Horn mf  C3     
    let values: [i32; 7] = [1430309, 1433942, 1433831, 1433937, 27734, 60, 0];
    sample_util::check(&sf.sample_headers[82], &values);

    // Fr Horn mf G2      
    let values: [i32; 7] = [1433974, 1439217, 1439071, 1439212, 27636, 60, 0];
    sample_util::check(&sf.sample_headers[83], &values);

    // FrHornmfD2 -52     
    let values: [i32; 7] = [1439249, 1446958, 1446764, 1446953, 27752, 60, 0];
    sample_util::check(&sf.sample_headers[84], &values);

    // FilterSnap
    let values: [i32; 7] = [1446990, 1447409, 1446997, 1447406, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[85], &values);

    // Dance Snare
    let values: [i32; 7] = [1447441, 1460100, 1447449, 1460100, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[86], &values);

    // Stacked Sawtooth-C62
    let values: [i32; 7] = [1460132, 1476698, 1460876, 1476603, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[87], &values);

    // Stacked Sawtooth-C42
    let values: [i32; 7] = [1476730, 1503558, 1479262, 1503284, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[88], &values);

    // Stacked Sawtooth-C63
    let values: [i32; 7] = [1503590, 1511874, 1503962, 1511826, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[89], &values);

    // Synth Strings 1-C42
    let values: [i32; 7] = [1511906, 1538150, 1513712, 1537735, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[90], &values);

    // Synth Strings 1-C63
    let values: [i32; 7] = [1538182, 1546461, 1538484, 1546348, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[91], &values);

    // Synth Strings 1-C62
    let values: [i32; 7] = [1546493, 1563049, 1547098, 1562826, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[92], &values);

    // Dist. Gt. Low-C3-ds
    let values: [i32; 7] = [1563081, 1584815, 1584282, 1584770, 16000, 60, 0];
    sample_util::check(&sf.sample_headers[93], &values);

    // Dist. Gt. Low-C3
    let values: [i32; 7] = [1584847, 1644751, 1643355, 1644362, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[94], &values);

    // Dist. Gt. Low-A2
    let values: [i32; 7] = [1644783, 1674189, 1671271, 1674068, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[95], &values);

    // Dist. Gt. Low-G2
    let values: [i32; 7] = [1674221, 1708269, 1706829, 1708170, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[96], &values);

    // Dist. Gt. Low-E2
    let values: [i32; 7] = [1708301, 1744781, 1744121, 1744654, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[97], &values);

    // Flute-A#6
    let values: [i32; 7] = [1744813, 1777514, 1766380, 1777348, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[98], &values);

    // Flute-G6
    let values: [i32; 7] = [1777546, 1814764, 1803197, 1814042, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[99], &values);

    // Flute-E6
    let values: [i32; 7] = [1814796, 1841853, 1830037, 1841732, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[100], &values);

    // Flute-C#6
    let values: [i32; 7] = [1841885, 1874424, 1868686, 1874270, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[101], &values);

    // Flute-A#5
    let values: [i32; 7] = [1874456, 1907009, 1895877, 1906872, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[102], &values);

    // Flute-G5
    let values: [i32; 7] = [1907041, 1945760, 1934352, 1945591, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[103], &values);

    // Flute-E5
    let values: [i32; 7] = [1945792, 1982021, 1970782, 1981834, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[104], &values);

    // Flute-C#5
    let values: [i32; 7] = [1982053, 2027109, 2015999, 2027070, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[105], &values);

    // Flute-A#4
    let values: [i32; 7] = [2027141, 2057112, 2045598, 2056944, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[106], &values);

    // Flute-G4
    let values: [i32; 7] = [2057144, 2097948, 2085574, 2097800, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[107], &values);

    // Flute-D#4
    let values: [i32; 7] = [2097980, 2121581, 2110472, 2121567, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[108], &values);

    // Flute-C#4
    let values: [i32; 7] = [2121613, 2151584, 2139035, 2151251, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[109], &values);

    // French Horn-E#4
    let values: [i32; 7] = [2151616, 2183611, 2173613, 2183526, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[110], &values);

    // French Horn-A#3
    let values: [i32; 7] = [2183643, 2213555, 2201794, 2213364, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[111], &values);

    // French Horn-E#3
    let values: [i32; 7] = [2213587, 2253270, 2232144, 2253158, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[112], &values);

    // French Horn-A#2
    let values: [i32; 7] = [2253302, 2317636, 2293040, 2315867, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[113], &values);

    // Muted Trumpet-F#4
    let values: [i32; 7] = [2317668, 2331300, 2329020, 2331261, 30637, 72, 0];
    sample_util::check(&sf.sample_headers[114], &values);

    // Synth Bass 2-F2
    let values: [i32; 7] = [2331332, 2344687, 2343798, 2344655, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[115], &values);

    // Synth Bass 2-B1
    let values: [i32; 7] = [2344719, 2368871, 2367219, 2368839, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[116], &values);

    // Synth Bass 2-F1
    let values: [i32; 7] = [2368903, 2391607, 2389283, 2391575, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[117], &values);

    // CelloAttack-A#3
    let values: [i32; 7] = [2391639, 2396452, 2391647, 2396444, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[118], &values);

    // CelloAttack-F#3
    let values: [i32; 7] = [2396484, 2401425, 2396492, 2401417, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[119], &values);

    // CelloAttack-D3
    let values: [i32; 7] = [2401457, 2407298, 2401465, 2407290, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[120], &values);

    // CelloAttack-A#2
    let values: [i32; 7] = [2407330, 2413014, 2407338, 2413006, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[121], &values);

    // CelloAttack-F#2
    let values: [i32; 7] = [2413046, 2419503, 2413054, 2419495, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[122], &values);

    // CelloAttack-D2
    let values: [i32; 7] = [2419535, 2424888, 2419543, 2424880, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[123], &values);

    // CelloAttack-A#1
    let values: [i32; 7] = [2424920, 2431089, 2424928, 2431081, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[124], &values);

    // CelloAttack-F#1
    let values: [i32; 7] = [2431121, 2436631, 2431129, 2436623, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[125], &values);

    // CelloAttack-D1
    let values: [i32; 7] = [2436663, 2441433, 2436671, 2441425, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[126], &values);

    // Cello-D#4
    let values: [i32; 7] = [2441465, 2476636, 2470788, 2476604, 30001, 75, 0];
    sample_util::check(&sf.sample_headers[127], &values);

    // Cello-B3
    let values: [i32; 7] = [2476668, 2506555, 2501157, 2506520, 30002, 71, 0];
    sample_util::check(&sf.sample_headers[128], &values);

    // Cello-A3
    let values: [i32; 7] = [2506587, 2536283, 2530446, 2536251, 29999, 69, 0];
    sample_util::check(&sf.sample_headers[129], &values);

    // Cello-F3
    let values: [i32; 7] = [2536315, 2559895, 2553819, 2559863, 29998, 65, 0];
    sample_util::check(&sf.sample_headers[130], &values);

    // Cello-E3
    let values: [i32; 7] = [2559927, 2587334, 2581699, 2587302, 30001, 64, 0];
    sample_util::check(&sf.sample_headers[131], &values);

    // Cello-B2
    let values: [i32; 7] = [2587366, 2624098, 2618357, 2624066, 29999, 59, 0];
    sample_util::check(&sf.sample_headers[132], &values);

    // Cello-F#2
    let values: [i32; 7] = [2624130, 2657750, 2651755, 2657718, 30000, 54, 0];
    sample_util::check(&sf.sample_headers[133], &values);

    // Cello-D#2
    let values: [i32; 7] = [2657782, 2690964, 2684960, 2690932, 29999, 51, 0];
    sample_util::check(&sf.sample_headers[134], &values);

    // Cello-A#1
    let values: [i32; 7] = [2690996, 2724370, 2718008, 2724338, 30002, 47, 0];
    sample_util::check(&sf.sample_headers[135], &values);

    // Cello-G#1
    let values: [i32; 7] = [2724402, 2761469, 2754754, 2761437, 28800, 44, 0];
    sample_util::check(&sf.sample_headers[136], &values);

    // Cello-F1
    let values: [i32; 7] = [2761501, 2794994, 2788815, 2794962, 30002, 41, 0];
    sample_util::check(&sf.sample_headers[137], &values);

    // Cello-D#1
    let values: [i32; 7] = [2795026, 2835980, 2829798, 2835948, 30000, 39, 0];
    sample_util::check(&sf.sample_headers[138], &values);

    // Cello-C#1
    let values: [i32; 7] = [2836012, 2876495, 2869939, 2876463, 29998, 37, 0];
    sample_util::check(&sf.sample_headers[139], &values);

    // VlnStrike-G6
    let values: [i32; 7] = [2876527, 2883257, 2876535, 2883249, 44100, 91, 0];
    sample_util::check(&sf.sample_headers[140], &values);

    // VlnStrike-E6
    let values: [i32; 7] = [2883289, 2889746, 2883297, 2889738, 44100, 88, 0];
    sample_util::check(&sf.sample_headers[141], &values);

    // VlnStrike-C6
    let values: [i32; 7] = [2889778, 2896798, 2889786, 2896790, 44100, 84, 0];
    sample_util::check(&sf.sample_headers[142], &values);

    // VlnStrike-A5
    let values: [i32; 7] = [2896830, 2903240, 2896838, 2903232, 44100, 81, 0];
    sample_util::check(&sf.sample_headers[143], &values);

    // VlnStrike-E5
    let values: [i32; 7] = [2903272, 2910330, 2903280, 2910322, 44100, 76, 0];
    sample_util::check(&sf.sample_headers[144], &values);

    // VlnStrike-A4
    let values: [i32; 7] = [2910362, 2920119, 2910370, 2920111, 44100, 69, 0];
    sample_util::check(&sf.sample_headers[145], &values);

    // VlnStrike-D4
    let values: [i32; 7] = [2920151, 2929165, 2920159, 2929157, 44100, 62, 0];
    sample_util::check(&sf.sample_headers[146], &values);

    // VlnStrike-A3
    let values: [i32; 7] = [2929197, 2936241, 2929205, 2936233, 44100, 57, 0];
    sample_util::check(&sf.sample_headers[147], &values);

    // Pulse 2-6000Hz
    let values: [i32; 7] = [2936273, 2936337, 2936285, 2936325, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[148], &values);

    // Pulse 2-3000Hz
    let values: [i32; 7] = [2936369, 2936433, 2936377, 2936425, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[149], &values);

    // Pulse 2-1500Hz
    let values: [i32; 7] = [2936465, 2936529, 2936481, 2936513, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[150], &values);

    // Pulse 2-750Hz
    let values: [i32; 7] = [2936561, 2936689, 2936593, 2936657, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[151], &values);

    // Pulse 2-375Hz
    let values: [i32; 7] = [2936721, 2936977, 2936785, 2936913, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[152], &values);

    // Pulse 2-187.5Hz
    let values: [i32; 7] = [2937009, 2937521, 2937137, 2937393, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[153], &values);

    // Pulse 2-93.75Hz
    let values: [i32; 7] = [2937553, 2938577, 2937809, 2938321, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[154], &values);

    // Slap Bass 2-E3P
    let values: [i32; 7] = [2938609, 2946860, 2946498, 2946855, 29762, 52, -19];
    sample_util::check(&sf.sample_headers[155], &values);

    // Slap Bass 2-F2P
    let values: [i32; 7] = [2946892, 2956139, 2955463, 2956136, 29762, 41, 0];
    sample_util::check(&sf.sample_headers[156], &values);

    // Slap Bass 2-F3F
    let values: [i32; 7] = [2956171, 2967031, 2966700, 2967025, 28409, 53, 3];
    sample_util::check(&sf.sample_headers[157], &values);

    // Slap Bass 2-A2F
    let values: [i32; 7] = [2967063, 2978974, 2978363, 2978899, 29762, 45, -23];
    sample_util::check(&sf.sample_headers[158], &values);

    // Acoustic Bass-C3
    let values: [i32; 7] = [2979006, 2992830, 2992141, 2992818, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[159], &values);

    // Clean Guitar-G4
    let values: [i32; 7] = [2992862, 3020268, 3018709, 3020236, 25000, 67, 0];
    sample_util::check(&sf.sample_headers[160], &values);

    // Clean Guitar-E5
    let values: [i32; 7] = [3020300, 3055818, 3054270, 3055786, 25000, 76, 0];
    sample_util::check(&sf.sample_headers[161], &values);

    // Clean Guitar-A#4
    let values: [i32; 7] = [3055850, 3075188, 3073710, 3075156, 25000, 70, 0];
    sample_util::check(&sf.sample_headers[162], &values);

    // Nylon Guitar-E4
    let values: [i32; 7] = [3075220, 3086865, 3086681, 3086815, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[163], &values);

    // Nylon Guitar-A4
    let values: [i32; 7] = [3086897, 3098532, 3098396, 3098496, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[164], &values);

    // Nylon Guitar-F4
    let values: [i32; 7] = [3098564, 3112183, 3112065, 3112128, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[165], &values);

    // Glockenspiel-G#4
    let values: [i32; 7] = [3112215, 3152017, 3142654, 3152012, 31250, 60, 0];
    sample_util::check(&sf.sample_headers[166], &values);

    // StrLoop - C6
    let values: [i32; 7] = [3152049, 3214068, 3167334, 3214062, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[167], &values);

    // StrLoop - A5
    let values: [i32; 7] = [3214100, 3262493, 3229116, 3261844, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[168], &values);

    // StrLoop - F#5
    let values: [i32; 7] = [3262525, 3314157, 3276985, 3314157, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[169], &values);

    // StrLoop - D#5
    let values: [i32; 7] = [3314189, 3361959, 3327605, 3361959, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[170], &values);

    // StrLoop - C5
    let values: [i32; 7] = [3361991, 3417526, 3379394, 3417524, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[171], &values);

    // StrLoop - A4
    let values: [i32; 7] = [3417558, 3473135, 3434252, 3473132, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[172], &values);

    // StrLoop - F#4
    let values: [i32; 7] = [3473167, 3519256, 3488798, 3519255, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[173], &values);

    // StrLoop - D#4
    let values: [i32; 7] = [3519288, 3567270, 3535114, 3567268, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[174], &values);

    // StrLoop - C4
    let values: [i32; 7] = [3567302, 3619904, 3583396, 3619903, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[175], &values);

    // StrLoop - A3
    let values: [i32; 7] = [3619936, 3664116, 3636299, 3664109, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[176], &values);

    // StrLoop - F#3
    let values: [i32; 7] = [3664148, 3716827, 3679774, 3716824, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[177], &values);

    // StrLoop - D#3
    let values: [i32; 7] = [3716859, 3773066, 3735867, 3773062, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[178], &values);

    // StrLoop - C3
    let values: [i32; 7] = [3773098, 3829761, 3791999, 3829759, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[179], &values);

    // StrLoop - A2
    let values: [i32; 7] = [3829793, 3902629, 3847085, 3902623, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[180], &values);

    // StrLoop - F#2
    let values: [i32; 7] = [3902661, 3948508, 3912673, 3947440, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[181], &values);

    // StrLoop - D#2
    let values: [i32; 7] = [3948540, 4012784, 3963823, 4012783, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[182], &values);

    // StrLoop - C2
    let values: [i32; 7] = [4012816, 4102278, 4030959, 4102276, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[183], &values);

    // StrLoop - A1
    let values: [i32; 7] = [4102310, 4159635, 4122679, 4159634, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[184], &values);

    // StrLoop - F#1
    let values: [i32; 7] = [4159667, 4237578, 4178995, 4235834, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[185], &values);

    // StrLoop - D#1
    let values: [i32; 7] = [4237610, 4310832, 4258279, 4310829, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[186], &values);

    // StrLoop - C1
    let values: [i32; 7] = [4310864, 4379479, 4330640, 4379477, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[187], &values);

    // Gtr Cut Down
    let values: [i32; 7] = [4379511, 4383423, 4379514, 4383416, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[188], &values);

    // Orchestral Cymbal
    let values: [i32; 7] = [4383455, 4446267, 4383463, 4446259, 27784, 60, 0];
    sample_util::check(&sf.sample_headers[189], &values);

    // Brushed Tom
    let values: [i32; 7] = [4446299, 4461302, 4446302, 4461295, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[190], &values);

    // Brush Swirl
    let values: [i32; 7] = [4461334, 4476337, 4461337, 4476330, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[191], &values);

    // Standard Snare 2
    let values: [i32; 7] = [4476369, 4490181, 4483332, 4490181, 29761, 38, 0];
    sample_util::check(&sf.sample_headers[192], &values);

    // Brush Snare
    let values: [i32; 7] = [4490213, 4492665, 4490216, 4492658, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[193], &values);

    // Jazz Snare Hard
    let values: [i32; 7] = [4492697, 4507747, 4492705, 4507739, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[194], &values);

    // Jazz Snare Soft 2
    let values: [i32; 7] = [4507779, 4511190, 4507787, 4511182, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[195], &values);

    // TR-909 Snare 1
    let values: [i32; 7] = [4511222, 4519061, 4511230, 4519053, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[196], &values);

    // TR-909 Snare 2
    let values: [i32; 7] = [4519093, 4531925, 4519101, 4531917, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[197], &values);

    // TR-909 Kick
    let values: [i32; 7] = [4531957, 4537701, 4531958, 4537701, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[198], &values);

    // TR-909 Scratch Pull
    let values: [i32; 7] = [4537733, 4542195, 4537741, 4542187, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[199], &values);

    // TR-909 Scratch Push
    let values: [i32; 7] = [4542227, 4546529, 4542235, 4546521, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[200], &values);

    // TR-808 Cowbell
    let values: [i32; 7] = [4546561, 4547549, 4547397, 4547541, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[201], &values);

    // TR-808 Cymbal
    let values: [i32; 7] = [4547581, 4589687, 4547589, 4589679, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[202], &values);

    // TR-808 Hat 3
    let values: [i32; 7] = [4589719, 4617511, 4589727, 4617503, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[203], &values);

    // TR-808 Hat 2
    let values: [i32; 7] = [4617543, 4620467, 4617551, 4620459, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[204], &values);

    // TR-808 Hat 1
    let values: [i32; 7] = [4620499, 4623633, 4620507, 4623625, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[205], &values);

    // TR-808 Clap
    let values: [i32; 7] = [4623665, 4628338, 4623673, 4628330, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[206], &values);

    // TR-808 Snare
    let values: [i32; 7] = [4628370, 4635366, 4628378, 4635358, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[207], &values);

    // TR-808 Kick
    let values: [i32; 7] = [4635398, 4642489, 4640742, 4642426, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[208], &values);

    // Synth Hand Clap
    let values: [i32; 7] = [4642521, 4654905, 4642529, 4654897, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[209], &values);

    // Electric Snare
    let values: [i32; 7] = [4654937, 4668040, 4654945, 4668032, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[210], &values);

    // Electronic Kick
    let values: [i32; 7] = [4668072, 4681302, 4668080, 4681294, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[211], &values);

    // Power Snare 1
    let values: [i32; 7] = [4681334, 4719382, 4681342, 4719374, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[212], &values);

    // Power Kick
    let values: [i32; 7] = [4719414, 4727745, 4719422, 4727737, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[213], &values);

    // Gated Kick
    let values: [i32; 7] = [4727777, 4743553, 4727785, 4743545, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[214], &values);

    // Power Snare 2
    let values: [i32; 7] = [4743585, 4756407, 4756000, 4756000, 29762, 47, 0];
    sample_util::check(&sf.sample_headers[215], &values);

    // Jazz Bass Drum
    let values: [i32; 7] = [4756439, 4763442, 4756442, 4763435, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[216], &values);

    // Room Snare
    let values: [i32; 7] = [4763474, 4772852, 4772794, 4772852, 29762, 43, 0];
    sample_util::check(&sf.sample_headers[217], &values);

    // Room Kick
    let values: [i32; 7] = [4772884, 4780134, 4772884, 4780134, 25000, 37, 0];
    sample_util::check(&sf.sample_headers[218], &values);

    // Standard Snare 4
    let values: [i32; 7] = [4780166, 4794404, 4780166, 4786849, 29762, 50, 0];
    sample_util::check(&sf.sample_headers[219], &values);

    // Standard Kick 1
    let values: [i32; 7] = [4794436, 4806469, 4794437, 4806469, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[220], &values);

    // Rim Shot
    let values: [i32; 7] = [4806501, 4812119, 4806509, 4812111, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[221], &values);

    // Chimes
    let values: [i32; 7] = [4812151, 4912665, 4894430, 4912655, 29762, 72, 0];
    sample_util::check(&sf.sample_headers[222], &values);

    // Standard Kick 2
    let values: [i32; 7] = [4912697, 4920720, 4912705, 4920712, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[223], &values);

    // Scratch Push
    let values: [i32; 7] = [4920752, 4928596, 4920760, 4928588, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[224], &values);

    // Hand Clap
    let values: [i32; 7] = [4928628, 4933170, 4928636, 4933162, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[225], &values);

    // Cowbell
    let values: [i32; 7] = [4933202, 4938033, 4933202, 4938033, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[226], &values);

    // Chinese Cymbal
    let values: [i32; 7] = [4938065, 4982383, 4975488, 4982375, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[227], &values);

    // Surdo Open
    let values: [i32; 7] = [4982415, 5055443, 4982423, 5055435, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[228], &values);

    // Surdo Muted
    let values: [i32; 7] = [5055475, 5064835, 5055483, 5064827, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[229], &values);

    // Jingle Bell
    let values: [i32; 7] = [5064867, 5084268, 5064875, 5084260, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[230], &values);

    // Shaker
    let values: [i32; 7] = [5084300, 5091391, 5084301, 5091391, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[231], &values);

    // Triangle (Perc)
    let values: [i32; 7] = [5091423, 5093491, 5091755, 5093483, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[232], &values);

    // Cuica Open
    let values: [i32; 7] = [5093523, 5106204, 5093531, 5106196, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[233], &values);

    // Cuica Mute
    let values: [i32; 7] = [5106236, 5115351, 5106244, 5115343, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[234], &values);

    // Claves
    let values: [i32; 7] = [5115383, 5117733, 5115384, 5117733, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[235], &values);

    // Guiro Long
    let values: [i32; 7] = [5117765, 5130816, 5117773, 5130808, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[236], &values);

    // Guiro Short
    let values: [i32; 7] = [5130848, 5132824, 5130856, 5132816, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[237], &values);

    // Whistle Long
    let values: [i32; 7] = [5132856, 5149544, 5132864, 5149536, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[238], &values);

    // Whistle Short
    let values: [i32; 7] = [5149576, 5153633, 5149584, 5153625, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[239], &values);

    // Maracas
    let values: [i32; 7] = [5153665, 5157262, 5153666, 5157262, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[240], &values);

    // Cabasa
    let values: [i32; 7] = [5157294, 5165727, 5157295, 5165727, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[241], &values);

    // Timbale Low
    let values: [i32; 7] = [5165759, 5180531, 5165767, 5180523, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[242], &values);

    // Timbale Rim
    let values: [i32; 7] = [5180563, 5192210, 5180564, 5192210, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[243], &values);

    // Tumba Low
    let values: [i32; 7] = [5192242, 5197041, 5192243, 5197041, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[244], &values);

    // Conga High Open
    let values: [i32; 7] = [5197073, 5206955, 5197081, 5206947, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[245], &values);

    // Conga High Muted
    let values: [i32; 7] = [5206987, 5209994, 5206988, 5209994, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[246], &values);

    // Bongo Tone
    let values: [i32; 7] = [5210026, 5212940, 5212851, 5212932, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[247], &values);

    // Bongo Rim
    let values: [i32; 7] = [5212972, 5215371, 5212975, 5214286, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[248], &values);

    // Vibra Slap
    let values: [i32; 7] = [5215403, 5269475, 5215404, 5269475, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[249], &values);

    // TR-808 Click
    let values: [i32; 7] = [5269507, 5270309, 5269510, 5270302, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[250], &values);

    // Square Click
    let values: [i32; 7] = [5270341, 5270471, 5270349, 5270463, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[251], &values);

    // Sticks
    let values: [i32; 7] = [5270503, 5279432, 5270511, 5279424, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[252], &values);

    // Slap
    let values: [i32; 7] = [5279464, 5285312, 5279465, 5285312, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[253], &values);

    // Dead Air
    let values: [i32; 7] = [5285344, 5285564, 5285352, 5285556, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[254], &values);

    // Hi-Hat Pedal
    let values: [i32; 7] = [5285596, 5290321, 5285604, 5290313, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[255], &values);

    // Hi-Hat Open Attack
    let values: [i32; 7] = [5290353, 5293062, 5290361, 5293054, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[256], &values);

    // Hi-Hat Closed Soft
    let values: [i32; 7] = [5293094, 5299638, 5293102, 5299630, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[257], &values);

    // Hi-Hat Closed Hard
    let values: [i32; 7] = [5299670, 5308730, 5299678, 5307055, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[258], &values);

    // Hi-Hat Open
    let values: [i32; 7] = [5308762, 5403756, 5381336, 5403742, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[259], &values);

    // Standard Snare 3
    let values: [i32; 7] = [5403788, 5420905, 5403788, 5410051, 29762, 47, 0];
    sample_util::check(&sf.sample_headers[260], &values);

    // Jazz Rim Shot
    let values: [i32; 7] = [5420937, 5426699, 5420945, 5426691, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[261], &values);

    // Jazz Snare Soft 1
    let values: [i32; 7] = [5426731, 5430205, 5426739, 5430197, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[262], &values);

    // Standard Snare 1
    let values: [i32; 7] = [5430237, 5441602, 5430237, 5441602, 29762, 45, 0];
    sample_util::check(&sf.sample_headers[263], &values);

    // Standard Kick 3
    let values: [i32; 7] = [5441634, 5445876, 5441634, 5445740, 29762, 38, 0];
    sample_util::check(&sf.sample_headers[264], &values);

    // Splash Cymbal
    let values: [i32; 7] = [5445908, 5498283, 5445916, 5498275, 28000, 55, 0];
    sample_util::check(&sf.sample_headers[265], &values);

    // Ride Bell
    let values: [i32; 7] = [5498315, 5578327, 5498323, 5578319, 28000, 53, 0];
    sample_util::check(&sf.sample_headers[266], &values);

    // DX7 Wave
    let values: [i32; 7] = [5578359, 5587712, 5580605, 5587681, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[267], &values);

    // DX7 Strike 6
    let values: [i32; 7] = [5587744, 5609782, 5608359, 5609692, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[268], &values);

    // DX7 Strike 5
    let values: [i32; 7] = [5609814, 5642867, 5640147, 5642738, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[269], &values);

    // DX7 Strike 4
    let values: [i32; 7] = [5642899, 5674211, 5673439, 5674077, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[270], &values);

    // DX7 Strike 3
    let values: [i32; 7] = [5674243, 5706583, 5706328, 5706516, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[271], &values);

    // DX7 Strike 2
    let values: [i32; 7] = [5706615, 5734535, 5732942, 5734444, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[272], &values);

    // DX7 Strike 1
    let values: [i32; 7] = [5734567, 5763242, 5762427, 5763177, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[273], &values);

    // Rhodes-Eb6 F
    let values: [i32; 7] = [5763274, 5766788, 5766704, 5766775, 44100, 99, 0];
    sample_util::check(&sf.sample_headers[274], &values);

    // Rhodes-Bb5 F
    let values: [i32; 7] = [5766820, 5772111, 5772019, 5772090, 44100, 94, 0];
    sample_util::check(&sf.sample_headers[275], &values);

    // Rhodes-Eb5 F
    let values: [i32; 7] = [5772143, 5782641, 5782440, 5782617, 44100, 87, 0];
    sample_util::check(&sf.sample_headers[276], &values);

    // Rhodes-Ab4 F
    let values: [i32; 7] = [5782673, 5790772, 5790706, 5790759, 44100, 80, 0];
    sample_util::check(&sf.sample_headers[277], &values);

    // Rhodes-F4 F
    let values: [i32; 7] = [5790804, 5795491, 5795392, 5795455, 44100, 77, 0];
    sample_util::check(&sf.sample_headers[278], &values);

    // Rhodes-B3 F
    let values: [i32; 7] = [5795523, 5803961, 5803733, 5803912, 44100, 71, 0];
    sample_util::check(&sf.sample_headers[279], &values);

    // Rhodes-E3 F
    let values: [i32; 7] = [5803993, 5808018, 5807818, 5807952, 44100, 64, 0];
    sample_util::check(&sf.sample_headers[280], &values);

    // Rhodes-G2 F
    let values: [i32; 7] = [5808050, 5815240, 5814973, 5815198, 44100, 55, 0];
    sample_util::check(&sf.sample_headers[281], &values);

    // Rhodes-C2 F
    let values: [i32; 7] = [5815272, 5820651, 5820246, 5820583, 44100, 48, 0];
    sample_util::check(&sf.sample_headers[282], &values);

    // Rhodes-F#1 F
    let values: [i32; 7] = [5820683, 5827881, 5827256, 5827731, 44100, 42, 0];
    sample_util::check(&sf.sample_headers[283], &values);

    // overtones 11
    let values: [i32; 7] = [5827913, 5828010, 5827963, 5827995, 48000, 60, 0];
    sample_util::check(&sf.sample_headers[284], &values);

    // Rhodes-Eb6 M
    let values: [i32; 7] = [5828042, 5831352, 5831259, 5831330, 44100, 99, 0];
    sample_util::check(&sf.sample_headers[285], &values);

    // Rhodes-Bb5 M
    let values: [i32; 7] = [5831384, 5833821, 5833726, 5833797, 44100, 94, 0];
    sample_util::check(&sf.sample_headers[286], &values);

    // Rhodes-Eb5 M
    let values: [i32; 7] = [5833853, 5840545, 5840268, 5840516, 44100, 87, 0];
    sample_util::check(&sf.sample_headers[287], &values);

    // Rhodes-Ab4 M
    let values: [i32; 7] = [5840577, 5842429, 5842339, 5842392, 44100, 80, 0];
    sample_util::check(&sf.sample_headers[288], &values);

    // Rhodes-F4 M
    let values: [i32; 7] = [5842461, 5847107, 5847020, 5847083, 44100, 77, 0];
    sample_util::check(&sf.sample_headers[289], &values);

    // Rhodes-B3 M
    let values: [i32; 7] = [5847139, 5857341, 5857141, 5857320, 44100, 71, 0];
    sample_util::check(&sf.sample_headers[290], &values);

    // Rhodes-E3 M
    let values: [i32; 7] = [5857373, 5869388, 5869087, 5869355, 44100, 64, 0];
    sample_util::check(&sf.sample_headers[291], &values);

    // Rhodes-G2 M
    let values: [i32; 7] = [5869420, 5876323, 5876033, 5876257, 44100, 55, 0];
    sample_util::check(&sf.sample_headers[292], &values);

    // Rhodes-C2 M
    let values: [i32; 7] = [5876355, 5883110, 5882665, 5883002, 44100, 48, 0];
    sample_util::check(&sf.sample_headers[293], &values);

    // Rhodes-F#1 M
    let values: [i32; 7] = [5883142, 5892132, 5891531, 5892010, 44100, 42, 0];
    sample_util::check(&sf.sample_headers[294], &values);

    // Rhodes-Eb6 P
    let values: [i32; 7] = [5892164, 5894320, 5894238, 5894309, 44100, 99, 0];
    sample_util::check(&sf.sample_headers[295], &values);

    // Rhodes-Bb5 P
    let values: [i32; 7] = [5894352, 5897314, 5897224, 5897295, 44100, 94, 0];
    sample_util::check(&sf.sample_headers[296], &values);

    // Rhodes-Eb5 P
    let values: [i32; 7] = [5897346, 5903682, 5903484, 5903661, 44100, 87, 0];
    sample_util::check(&sf.sample_headers[297], &values);

    // Rhodes-Ab4 P
    let values: [i32; 7] = [5903714, 5906299, 5906224, 5906277, 44100, 80, 0];
    sample_util::check(&sf.sample_headers[298], &values);

    // Rhodes-F4 P
    let values: [i32; 7] = [5906331, 5910536, 5910459, 5910522, 44100, 77, 0];
    sample_util::check(&sf.sample_headers[299], &values);

    // Rhodes-B3 P
    let values: [i32; 7] = [5910568, 5917605, 5917401, 5917580, 44100, 71, 0];
    sample_util::check(&sf.sample_headers[300], &values);

    // Rhodes-E3 P
    let values: [i32; 7] = [5917637, 5927324, 5926885, 5927286, 44100, 64, 0];
    sample_util::check(&sf.sample_headers[301], &values);

    // Rhodes-G2 P
    let values: [i32; 7] = [5927356, 5934368, 5934007, 5934231, 44100, 55, 0];
    sample_util::check(&sf.sample_headers[302], &values);

    // Rhodes-C2 P
    let values: [i32; 7] = [5934400, 5941951, 5941438, 5941776, 44100, 48, 0];
    sample_util::check(&sf.sample_headers[303], &values);

    // Rhodes-D1 P
    let values: [i32; 7] = [5941983, 5952253, 5951338, 5951937, 44100, 38, 0];
    sample_util::check(&sf.sample_headers[304], &values);

    // Grand Piano-D#3
    let values: [i32; 7] = [5952285, 6075605, 6069041, 6075573, 44100, 63, 0];
    sample_util::check(&sf.sample_headers[305], &values);

    // Grand Piano-C7
    let values: [i32; 7] = [6075637, 6099137, 6094933, 6099137, 30767, 96, 0];
    sample_util::check(&sf.sample_headers[306], &values);

    // Grand Piano-G6
    let values: [i32; 7] = [6099169, 6118250, 6109550, 6118230, 30380, 91, 0];
    sample_util::check(&sf.sample_headers[307], &values);

    // Grand Piano-D6
    let values: [i32; 7] = [6118282, 6149940, 6135643, 6149940, 31128, 86, 0];
    sample_util::check(&sf.sample_headers[308], &values);

    // Grand Piano-A5
    let values: [i32; 7] = [6149972, 6195937, 6177408, 6195937, 30536, 81, 0];
    sample_util::check(&sf.sample_headers[309], &values);

    // Grand Piano-D#5
    let values: [i32; 7] = [6195969, 6269620, 6234366, 6269615, 31113, 75, 0];
    sample_util::check(&sf.sample_headers[310], &values);

    // Grand Piano-C5
    let values: [i32; 7] = [6269652, 6329168, 6303597, 6329168, 30982, 72, 0];
    sample_util::check(&sf.sample_headers[311], &values);

    // Grand Piano-F#4
    let values: [i32; 7] = [6329200, 6380496, 6357631, 6380496, 32014, 66, 0];
    sample_util::check(&sf.sample_headers[312], &values);

    // Grand Piano-C4
    let values: [i32; 7] = [6380528, 6443054, 6411398, 6443054, 31000, 60, 0];
    sample_util::check(&sf.sample_headers[313], &values);

    // Grand Piano-A#3
    let values: [i32; 7] = [6443086, 6505704, 6475190, 6505704, 30930, 58, 0];
    sample_util::check(&sf.sample_headers[314], &values);

    // Grand Piano-F3
    let values: [i32; 7] = [6505736, 6569143, 6538586, 6569142, 31000, 53, 0];
    sample_util::check(&sf.sample_headers[315], &values);

    // Grand Piano-C3
    let values: [i32; 7] = [6569175, 6647229, 6613096, 6647229, 31000, 48, 0];
    sample_util::check(&sf.sample_headers[316], &values);

    // Grand Piano-G#2
    let values: [i32; 7] = [6647261, 6725115, 6686507, 6725115, 30960, 44, 0];
    sample_util::check(&sf.sample_headers[317], &values);

    // Grand Piano-D2
    let values: [i32; 7] = [6725147, 6806065, 6774465, 6806065, 32000, 38, 0];
    sample_util::check(&sf.sample_headers[318], &values);

    // Grand Piano-A#1
    let values: [i32; 7] = [6806097, 6887710, 6856632, 6887710, 30970, 34, 0];
    sample_util::check(&sf.sample_headers[319], &values);

    // Grand Piano-F1
    let values: [i32; 7] = [6887742, 6967672, 6935611, 6967671, 31000, 29, 0];
    sample_util::check(&sf.sample_headers[320], &values);

    // Grand Piano-D1
    let values: [i32; 7] = [6967704, 7038491, 7003344, 7038490, 31000, 26, 0];
    sample_util::check(&sf.sample_headers[321], &values);

    // Sine-Triangle-12000H
    let values: [i32; 7] = [7038523, 7038571, 7038529, 7038565, 48000, 126, 0];
    sample_util::check(&sf.sample_headers[322], &values);

    // Triangle-6000Hz
    let values: [i32; 7] = [7038603, 7038667, 7038615, 7038655, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[323], &values);

    // Triangle-3000Hz
    let values: [i32; 7] = [7038699, 7038763, 7038707, 7038755, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[324], &values);

    // Sine-6000Hz
    let values: [i32; 7] = [7038795, 7038859, 7038807, 7038847, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[325], &values);

    // Sine-3000Hz
    let values: [i32; 7] = [7038891, 7038955, 7038899, 7038947, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[326], &values);

    // Sine-1500Hz
    let values: [i32; 7] = [7038987, 7039051, 7039003, 7039035, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[327], &values);

    // Sine-750Hz
    let values: [i32; 7] = [7039083, 7039211, 7039115, 7039179, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[328], &values);

    // Sine-375Hz
    let values: [i32; 7] = [7039243, 7039499, 7039307, 7039435, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[329], &values);

    // Sine-187.5Hz
    let values: [i32; 7] = [7039531, 7040043, 7039659, 7039915, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[330], &values);

    // Sine-93.75Hz
    let values: [i32; 7] = [7040075, 7041099, 7040331, 7040843, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[331], &values);

    // Pulse 1-3000Hz
    let values: [i32; 7] = [7041131, 7041195, 7041139, 7041187, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[332], &values);

    // Pulse 1-1500Hz
    let values: [i32; 7] = [7041227, 7041291, 7041243, 7041275, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[333], &values);

    // Pulse 1-750Hz
    let values: [i32; 7] = [7041323, 7041451, 7041355, 7041419, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[334], &values);

    // Pulse 1-375Hz
    let values: [i32; 7] = [7041483, 7041739, 7041547, 7041675, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[335], &values);

    // Pulse 1-187.5Hz
    let values: [i32; 7] = [7041771, 7042283, 7041899, 7042155, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[336], &values);

    // Pulse 1-93.75Hz
    let values: [i32; 7] = [7042315, 7043339, 7042571, 7043083, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[337], &values);

    // KPianoB1
    let values: [i32; 7] = [7043371, 7060602, 7054026, 7060598, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[338], &values);

    // KPianoG2
    let values: [i32; 7] = [7060634, 7082764, 7078271, 7082761, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[339], &values);

    // KPianoCx4
    let values: [i32; 7] = [7082796, 7104369, 7104302, 7104365, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[340], &values);

    // KPianoB4
    let values: [i32; 7] = [7104401, 7129617, 7119829, 7129613, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[341], &values);

    // KPianoF5
    let values: [i32; 7] = [7129649, 7136628, 7136497, 7136623, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[342], &values);

    // KPianoDx5
    let values: [i32; 7] = [7136660, 7142805, 7142753, 7142801, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[343], &values);

    // LeFone
    let values: [i32; 7] = [7142837, 7144421, 7142844, 7144417, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[344], &values);

    // TBellD4Wave
    let values: [i32; 7] = [7144453, 7149869, 7149282, 7149865, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[345], &values);

    // BSawtoothWaveA3
    let values: [i32; 7] = [7149901, 7149970, 7149916, 7149966, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[346], &values);

    // ColdGlass7Wave
    let values: [i32; 7] = [7150002, 7150071, 7150019, 7150067, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[347], &values);

    // ChanterAx1
    let values: [i32; 7] = [7150103, 7151960, 7151908, 7151956, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[348], &values);

    // AcGtrB3
    let values: [i32; 7] = [7151992, 7158232, 7158160, 7158228, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[349], &values);

    // JazzGuitLoop
    let values: [i32; 7] = [7158264, 7158382, 7158273, 7158378, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[350], &values);

    // SynthBassLoop
    let values: [i32; 7] = [7158414, 7158854, 7158423, 7158850, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[351], &values);

    // VibesE2
    let values: [i32; 7] = [7158886, 7159667, 7159582, 7159663, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[352], &values);

    // ColdGlass12Wave
    let values: [i32; 7] = [7159699, 7159789, 7159714, 7159785, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[353], &values);

    // GlockLoopC4
    let values: [i32; 7] = [7159821, 7160036, 7159828, 7160032, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[354], &values);

    // MarimbaC3
    let values: [i32; 7] = [7160068, 7160884, 7160856, 7160880, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[355], &values);

    // EPiano2MS
    let values: [i32; 7] = [7160916, 7162088, 7162036, 7162084, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[356], &values);

    // Triangle
    let values: [i32; 7] = [7162120, 7164188, 7162456, 7164184, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[357], &values);

    // Vibraloop
    let values: [i32; 7] = [7164220, 7165007, 7164229, 7165003, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[358], &values);

    // XyloE4Looped
    let values: [i32; 7] = [7165039, 7166018, 7165046, 7166015, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[359], &values);

    // SquareWave
    let values: [i32; 7] = [7166050, 7168476, 7168418, 7168473, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[360], &values);

    // SawStackWaveMS
    let values: [i32; 7] = [7168508, 7182256, 7168797, 7182253, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[361], &values);

    // SynthStringsC4
    let values: [i32; 7] = [7182288, 7192254, 7183560, 7192250, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[362], &values);

    // WhiteNoiseWave
    let values: [i32; 7] = [7192286, 7200579, 7192293, 7200575, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[363], &values);

    // OohVoiceC3
    let values: [i32; 7] = [7200611, 7209712, 7200646, 7209708, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[364], &values);

    // OcarinaFx2
    let values: [i32; 7] = [7209744, 7210930, 7210888, 7210926, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[365], &values);

    // KPianoB5
    let values: [i32; 7] = [7210962, 7216360, 7213925, 7216356, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[366], &values);

    // NGuitrF2
    let values: [i32; 7] = [7216392, 7220220, 7220119, 7220216, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[367], &values);

    // Square-12000Hz
    let values: [i32; 7] = [7220252, 7220300, 7220258, 7220294, 48000, 126, 0];
    sample_util::check(&sf.sample_headers[368], &values);

    // Square-6000Hz
    let values: [i32; 7] = [7220332, 7220396, 7220344, 7220384, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[369], &values);

    // Square-3000Hz
    let values: [i32; 7] = [7220428, 7220492, 7220436, 7220484, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[370], &values);

    // Saw Incline-12000Hz
    let values: [i32; 7] = [7220524, 7220572, 7220530, 7220566, 48000, 126, 0];
    sample_util::check(&sf.sample_headers[371], &values);

    // Saw Incline-6000Hz
    let values: [i32; 7] = [7220604, 7220668, 7220616, 7220656, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[372], &values);

    // Saw Incline-3000Hz
    let values: [i32; 7] = [7220700, 7220764, 7220708, 7220756, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[373], &values);

    // Saw Decline-12000Hz
    let values: [i32; 7] = [7220796, 7220844, 7220802, 7220838, 48000, 126, 0];
    sample_util::check(&sf.sample_headers[374], &values);

    // Saw Decline-6000Hz
    let values: [i32; 7] = [7220876, 7220940, 7220888, 7220928, 48000, 114, 0];
    sample_util::check(&sf.sample_headers[375], &values);

    // Saw Decline-3000Hz
    let values: [i32; 7] = [7220972, 7221036, 7220980, 7221028, 48000, 102, 0];
    sample_util::check(&sf.sample_headers[376], &values);

    // Synth Strings 1-C4
    let values: [i32; 7] = [7221068, 7273553, 7224680, 7272726, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[377], &values);

    // Synth Strings 1-C6
    let values: [i32; 7] = [7273585, 7306693, 7274796, 7306251, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[378], &values);

    // Clean Guitar-G3
    let values: [i32; 7] = [7306725, 7349828, 7348268, 7349796, 25000, 55, 0];
    sample_util::check(&sf.sample_headers[379], &values);

    // Clean Guitar-E4
    let values: [i32; 7] = [7349860, 7385410, 7384016, 7385378, 25000, 64, 0];
    sample_util::check(&sf.sample_headers[380], &values);

    // Clean Guitar-B3
    let values: [i32; 7] = [7385442, 7415259, 7413712, 7415227, 25000, 59, 0];
    sample_util::check(&sf.sample_headers[381], &values);

    // Marimba-C6
    let values: [i32; 7] = [7415291, 7421628, 7421542, 7421604, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[382], &values);

    // Marimba-C4
    let values: [i32; 7] = [7421660, 7439124, 7438890, 7439060, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[383], &values);

    // Harpsichord-F6
    let values: [i32; 7] = [7439156, 7445408, 7445313, 7445376, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[384], &values);

    // Harpsichord-F5
    let values: [i32; 7] = [7445440, 7457943, 7457630, 7457881, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[385], &values);

    // Harpsichord-F4
    let values: [i32; 7] = [7457975, 7485677, 7485360, 7485611, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[386], &values);

    // Harpsichord-F3
    let values: [i32; 7] = [7485709, 7511826, 7511535, 7511786, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[387], &values);

    // Harpsichord-C#2
    let values: [i32; 7] = [7511858, 7532355, 7531630, 7532261, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[388], &values);

    // Harpsichord-A2
    let values: [i32; 7] = [7532387, 7552653, 7551774, 7552572, 44101, 60, 0];
    sample_util::check(&sf.sample_headers[389], &values);

    // Carillon
    let values: [i32; 7] = [7552685, 7553883, 7552977, 7553849, 22050, 84, 0];
    sample_util::check(&sf.sample_headers[390], &values);

    // Car-Crash
    let values: [i32; 7] = [7553915, 7598904, 7553923, 7598896, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[391], &values);

    // Triangle-93.75Hz
    let values: [i32; 7] = [7598936, 7599960, 7599192, 7599704, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[392], &values);

    // Triangle-750Hz
    let values: [i32; 7] = [7599992, 7600120, 7600024, 7600088, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[393], &values);

    // Triangle-375Hz
    let values: [i32; 7] = [7600152, 7600408, 7600216, 7600344, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[394], &values);

    // Triangle-187.5Hz
    let values: [i32; 7] = [7600440, 7600952, 7600568, 7600824, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[395], &values);

    // Triangle-1500Hz
    let values: [i32; 7] = [7600984, 7601048, 7601000, 7601032, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[396], &values);

    // Square-93.75Hz
    let values: [i32; 7] = [7601080, 7602104, 7601336, 7601848, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[397], &values);

    // Square-750Hz
    let values: [i32; 7] = [7602136, 7602264, 7602168, 7602232, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[398], &values);

    // Square-375Hz
    let values: [i32; 7] = [7602296, 7602552, 7602360, 7602488, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[399], &values);

    // Square-187.5Hz
    let values: [i32; 7] = [7602584, 7603096, 7602712, 7602968, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[400], &values);

    // Square-1500Hz
    let values: [i32; 7] = [7603128, 7603192, 7603144, 7603176, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[401], &values);

    // Saw Incline-93.75Hz
    let values: [i32; 7] = [7603224, 7604248, 7603480, 7603992, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[402], &values);

    // Saw Incline-750Hz
    let values: [i32; 7] = [7604280, 7604408, 7604312, 7604376, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[403], &values);

    // Saw Incline-375Hz
    let values: [i32; 7] = [7604440, 7604696, 7604504, 7604632, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[404], &values);

    // Saw Incline-187.5Hz
    let values: [i32; 7] = [7604728, 7605240, 7604856, 7605112, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[405], &values);

    // Saw Incline-1500Hz
    let values: [i32; 7] = [7605272, 7605336, 7605288, 7605320, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[406], &values);

    // Saw Decline-93.75Hz
    let values: [i32; 7] = [7605368, 7606392, 7605624, 7606136, 48000, 42, 0];
    sample_util::check(&sf.sample_headers[407], &values);

    // Saw Decline-750Hz
    let values: [i32; 7] = [7606424, 7606552, 7606456, 7606520, 48000, 78, 0];
    sample_util::check(&sf.sample_headers[408], &values);

    // Saw Decline-375Hz
    let values: [i32; 7] = [7606584, 7606840, 7606648, 7606776, 48000, 66, 0];
    sample_util::check(&sf.sample_headers[409], &values);

    // Saw Decline-187.5Hz
    let values: [i32; 7] = [7606872, 7607384, 7607000, 7607256, 48000, 54, 0];
    sample_util::check(&sf.sample_headers[410], &values);

    // Saw Decline-1500Hz
    let values: [i32; 7] = [7607416, 7607480, 7607432, 7607464, 48000, 90, 0];
    sample_util::check(&sf.sample_headers[411], &values);

    // Gtr Click
    let values: [i32; 7] = [7607512, 7607786, 7607515, 7607779, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[412], &values);

    // Scratch Pull
    let values: [i32; 7] = [7607818, 7617975, 7607826, 7617967, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[413], &values);

    // Crystal-C5
    let values: [i32; 7] = [7618007, 7622516, 7622280, 7622447, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[414], &values);

    // Bassoon-B4
    let values: [i32; 7] = [7622548, 7627153, 7626854, 7627120, 33000, 71, 0];
    sample_util::check(&sf.sample_headers[415], &values);

    // Bassoon-D5
    let values: [i32; 7] = [7627185, 7632880, 7632408, 7632861, 33000, 74, 0];
    sample_util::check(&sf.sample_headers[416], &values);

    // Birds
    let values: [i32; 7] = [7632912, 7698335, 7632922, 7698326, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[417], &values);

    // Stream
    let values: [i32; 7] = [7698367, 7769150, 7698375, 7769142, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[418], &values);

    // Punch
    let values: [i32; 7] = [7769182, 7772143, 7769190, 7771670, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[419], &values);

    // Bone Crunch
    let values: [i32; 7] = [7772175, 7784201, 7772183, 7784193, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[420], &values);

    // Orchestral Harp-C4
    let values: [i32; 7] = [7784233, 7799383, 7797204, 7799351, 21604, 60, 0];
    sample_util::check(&sf.sample_headers[421], &values);

    // Orchestral Harp-E3
    let values: [i32; 7] = [7799415, 7817764, 7815632, 7817732, 21605, 52, 0];
    sample_util::check(&sf.sample_headers[422], &values);

    // Orchestral Harp-A6
    let values: [i32; 7] = [7817796, 7850993, 7844752, 7850961, 44101, 93, 0];
    sample_util::check(&sf.sample_headers[423], &values);

    // Orchestral Harp-F#5
    let values: [i32; 7] = [7851025, 7893912, 7891173, 7893880, 44101, 78, 0];
    sample_util::check(&sf.sample_headers[424], &values);

    // Orchestral Harp-C#4
    let values: [i32; 7] = [7893944, 7936951, 7934199, 7936919, 44101, 61, 0];
    sample_util::check(&sf.sample_headers[425], &values);

    // Acoustic Bass-C5
    let values: [i32; 7] = [7936983, 7939135, 7939091, 7939125, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[426], &values);

    // Acoustic Bass-G#3
    let values: [i32; 7] = [7939167, 7945047, 7944918, 7945004, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[427], &values);

    // Acoustic Bass-G#2
    let values: [i32; 7] = [7945079, 7955581, 7955258, 7955525, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[428], &values);

    // Double Bass-G3
    let values: [i32; 7] = [7955613, 7975487, 7970207, 7975455, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[429], &values);

    // Double Bass-D3
    let values: [i32; 7] = [7975519, 7994737, 7988026, 7994705, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[430], &values);

    // Double Bass-C3
    let values: [i32; 7] = [7994769, 8016832, 8010334, 8016800, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[431], &values);

    // Double Bass-F#2
    let values: [i32; 7] = [8016864, 8035581, 8028377, 8035549, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[432], &values);

    // Double Bass-D#2
    let values: [i32; 7] = [8035613, 8055639, 8048756, 8055607, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[433], &values);

    // Double Bass-A#1
    let values: [i32; 7] = [8055671, 8078088, 8071202, 8078056, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[434], &values);

    // Double Bass-G1
    let values: [i32; 7] = [8078120, 8092232, 8085800, 8092200, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[435], &values);

    // Breath Noise-2
    let values: [i32; 7] = [8092264, 8094384, 8092272, 8094376, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[436], &values);

    // Breath Noise-1
    let values: [i32; 7] = [8094416, 8101833, 8094424, 8101833, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[437], &values);

    // Reverse Cymbal
    let values: [i32; 7] = [8101865, 8141206, 8101873, 8141198, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[438], &values);

    // Electric Tom
    let values: [i32; 7] = [8141238, 8156236, 8141239, 8156236, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[439], &values);

    // Solo Vox-C5
    let values: [i32; 7] = [8156268, 8160131, 8156290, 8160111, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[440], &values);

    // Shamisen-C#4
    let values: [i32; 7] = [8160163, 8161340, 8161218, 8161308, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[441], &values);

    // Shamisen-B3
    let values: [i32; 7] = [8161372, 8162487, 8162404, 8162455, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[442], &values);

    // Shamisen-F#3
    let values: [i32; 7] = [8162519, 8163596, 8163497, 8163564, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[443], &values);

    // Shamisen-C3
    let values: [i32; 7] = [8163628, 8164857, 8164730, 8164825, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[444], &values);

    // Shamisen-F#2
    let values: [i32; 7] = [8164889, 8166129, 8165962, 8166097, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[445], &values);

    // Steel Drums-C4
    let values: [i32; 7] = [8166161, 8192071, 8191895, 8192009, 29761, 60, 0];
    sample_util::check(&sf.sample_headers[446], &values);

    // Scream
    let values: [i32; 7] = [8192103, 8217306, 8206689, 8217306, 29761, 60, 0];
    sample_util::check(&sf.sample_headers[447], &values);

    // Explosion
    let values: [i32; 7] = [8217338, 8235930, 8217346, 8235922, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[448], &values);

    // Ocarina-F#6
    let values: [i32; 7] = [8235962, 8236311, 8236293, 8236303, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[449], &values);

    // White Noise
    let values: [i32; 7] = [8236343, 8269099, 8236360, 8269076, 48000, 60, 0];
    sample_util::check(&sf.sample_headers[450], &values);

    // Ocarina-B5
    let values: [i32; 7] = [8269131, 8273937, 8271512, 8271541, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[451], &values);

    // Ocarina-G5
    let values: [i32; 7] = [8273969, 8277880, 8275614, 8275642, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[452], &values);

    // Ocarina-E5
    let values: [i32; 7] = [8277912, 8282098, 8279826, 8279859, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[453], &values);

    // Ocarina-C5
    let values: [i32; 7] = [8282130, 8286882, 8284058, 8284102, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[454], &values);

    // Concert Choir-C6
    let values: [i32; 7] = [8286914, 8291940, 8286932, 8291932, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[455], &values);

    // Synth Voice Oohs-C4
    let values: [i32; 7] = [8291972, 8314953, 8291986, 8314945, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[456], &values);

    // Synth Voice Oohs-F#3
    let values: [i32; 7] = [8314985, 8329739, 8315052, 8329731, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[457], &values);

    // Synth Voice Oohs-C3
    let values: [i32; 7] = [8329771, 8340919, 8329833, 8340911, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[458], &values);

    // Organ Slow-A4
    let values: [i32; 7] = [8340951, 8346168, 8341418, 8341441, 44100, 65, 0];
    sample_util::check(&sf.sample_headers[459], &values);

    // Voice Loop-C5
    let values: [i32; 7] = [8346200, 8357631, 8346223, 8357623, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[460], &values);

    // Voice Loop-F4
    let values: [i32; 7] = [8357663, 8367169, 8357669, 8367161, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[461], &values);

    // Voice Loop-A3
    let values: [i32; 7] = [8367201, 8374183, 8367205, 8374175, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[462], &values);

    // Voice Loop-Db3
    let values: [i32; 7] = [8374215, 8381403, 8374311, 8381395, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[463], &values);

    // Siren
    let values: [i32; 7] = [8381435, 8389445, 8381449, 8389413, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[464], &values);

    // Telephone 2
    let values: [i32; 7] = [8389477, 8392662, 8390557, 8392652, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[465], &values);

    // Harpsichord-C6
    let values: [i32; 7] = [8392694, 8393353, 8393333, 8393345, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[466], &values);

    // Starship Noise
    let values: [i32; 7] = [8393385, 8402613, 8396010, 8402605, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[467], &values);

    // Jet
    let values: [i32; 7] = [8402645, 8426211, 8402719, 8426203, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[468], &values);

    // Train
    let values: [i32; 7] = [8426243, 8454989, 8426256, 8454982, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[469], &values);

    // Foot Step
    let values: [i32; 7] = [8455021, 8458820, 8455024, 8458813, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[470], &values);

    // Windchimes
    let values: [i32; 7] = [8458852, 8481977, 8458862, 8481970, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[471], &values);

    // Bubbles
    let values: [i32; 7] = [8482009, 8508892, 8482054, 8508886, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[472], &values);

    // Heartbeat
    let values: [i32; 7] = [8508924, 8529980, 8508932, 8529972, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[473], &values);

    // Gun Shot
    let values: [i32; 7] = [8530012, 8540432, 8530015, 8540425, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[474], &values);

    // Car-Pass
    let values: [i32; 7] = [8540464, 8583141, 8580969, 8583134, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[475], &values);

    // Door Slam
    let values: [i32; 7] = [8583173, 8596658, 8583176, 8596651, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[476], &values);

    // Car-Skid
    let values: [i32; 7] = [8596690, 8617481, 8596693, 8617474, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[477], &values);

    // Door Creak
    let values: [i32; 7] = [8617513, 8646128, 8617516, 8646121, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[478], &values);

    // Horse Gallop
    let values: [i32; 7] = [8646160, 8666539, 8646168, 8666531, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[479], &values);

    // Thunder
    let values: [i32; 7] = [8666571, 8705935, 8666578, 8705626, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[480], &values);

    // Gtr Slap
    let values: [i32; 7] = [8705967, 8709106, 8705970, 8709099, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[481], &values);

    // Laughing
    let values: [i32; 7] = [8709138, 8733806, 8709141, 8733799, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[482], &values);

    // Car-Start
    let values: [i32; 7] = [8733838, 8772438, 8765575, 8772430, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[483], &values);

    // Dog
    let values: [i32; 7] = [8772470, 8782176, 8772473, 8782169, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[484], &values);

    // Rain
    let values: [i32; 7] = [8782208, 8855609, 8782240, 8855577, 30000, 60, 0];
    sample_util::check(&sf.sample_headers[485], &values);

    // Gtr Cut Up
    let values: [i32; 7] = [8855641, 8858516, 8855644, 8858509, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[486], &values);

    // Distortion Guitar-G3
    let values: [i32; 7] = [8858548, 8883727, 8883537, 8883664, 24996, 55, 0];
    sample_util::check(&sf.sample_headers[487], &values);

    // Distortion Guitar-E5
    let values: [i32; 7] = [8883759, 8902310, 8902148, 8902239, 30000, 76, 0];
    sample_util::check(&sf.sample_headers[488], &values);

    // Distortion Guitar-E4
    let values: [i32; 7] = [8902342, 8928388, 8928213, 8928289, 24994, 64, 0];
    sample_util::check(&sf.sample_headers[489], &values);

    // Distortion Guitar-E2
    let values: [i32; 7] = [8928420, 8944996, 8944692, 8944875, 14998, 40, 0];
    sample_util::check(&sf.sample_headers[490], &values);

    // Distortion Guitar-B4
    let values: [i32; 7] = [8945028, 8967474, 8967128, 8967310, 29985, 71, 0];
    sample_util::check(&sf.sample_headers[491], &values);

    // Distortion Guitar-B3
    let values: [i32; 7] = [8967506, 8996142, 8995905, 8996006, 24993, 59, 0];
    sample_util::check(&sf.sample_headers[492], &values);

    // Distortion Guitar-A5
    let values: [i32; 7] = [8996174, 9018785, 9018495, 9018734, 29992, 81, 0];
    sample_util::check(&sf.sample_headers[493], &values);

    // Distortion Guitar-G4
    let values: [i32; 7] = [9018817, 9043959, 9043550, 9043795, 31990, 67, 0];
    sample_util::check(&sf.sample_headers[494], &values);

    // Concert Choir-G5
    let values: [i32; 7] = [9043991, 9076755, 9053865, 9074349, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[495], &values);

    // Concert Choir-F5
    let values: [i32; 7] = [9076787, 9108257, 9089018, 9106972, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[496], &values);

    // Concert Choir-C#5
    let values: [i32; 7] = [9108289, 9139548, 9124977, 9137829, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[497], &values);

    // Concert Choir-B4
    let values: [i32; 7] = [9139580, 9172499, 9157159, 9171804, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[498], &values);

    // Concert Choir-A4
    let values: [i32; 7] = [9172531, 9200914, 9183452, 9200889, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[499], &values);

    // Concert Choir-F4
    let values: [i32; 7] = [9200946, 9228807, 9212801, 9228754, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[500], &values);

    // Concert Choir-C#4
    let values: [i32; 7] = [9228839, 9262321, 9245973, 9262289, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[501], &values);

    // Concert Choir-B3
    let values: [i32; 7] = [9262353, 9293658, 9277494, 9293644, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[502], &values);

    // Concert Choir-G3
    let values: [i32; 7] = [9293690, 9325619, 9307294, 9325577, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[503], &values);

    // Concert Choir-D#3
    let values: [i32; 7] = [9325651, 9353240, 9335770, 9353165, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[504], &values);

    // Concert Choir-C#3
    let values: [i32; 7] = [9353272, 9385018, 9370699, 9384974, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[505], &values);

    // Concert Choir-A2
    let values: [i32; 7] = [9385050, 9418592, 9401395, 9418542, 28000, 60, 0];
    sample_util::check(&sf.sample_headers[506], &values);

    // CP-80 EP-G6
    let values: [i32; 7] = [9418624, 9425241, 9424930, 9425218, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[507], &values);

    // CP-80 EP-C6
    let values: [i32; 7] = [9425273, 9431391, 9431354, 9431375, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[508], &values);

    // CP-80 EP-E5
    let values: [i32; 7] = [9431423, 9439478, 9439356, 9439456, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[509], &values);

    // CP-80 EP-G4
    let values: [i32; 7] = [9439510, 9449701, 9449645, 9449673, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[510], &values);

    // CP-80 EP-C4
    let values: [i32; 7] = [9449733, 9459274, 9459212, 9459254, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[511], &values);

    // CP-80 EP-E3
    let values: [i32; 7] = [9459306, 9472043, 9471758, 9472025, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[512], &values);

    // CP-80 EP-G2
    let values: [i32; 7] = [9472075, 9484906, 9484532, 9484870, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[513], &values);

    // CP-80 EP-C2
    let values: [i32; 7] = [9484938, 9495235, 9495014, 9495182, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[514], &values);

    // Harmonica-B5
    let values: [i32; 7] = [9495267, 9500033, 9499974, 9500006, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[515], &values);

    // Harmonica-E5
    let values: [i32; 7] = [9500065, 9504437, 9504355, 9504427, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[516], &values);

    // Harmonica-A4
    let values: [i32; 7] = [9504469, 9509450, 9509269, 9509414, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[517], &values);

    // Harmonica-E4
    let values: [i32; 7] = [9509482, 9513868, 9513749, 9513845, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[518], &values);

    // Harmonica-A3
    let values: [i32; 7] = [9513900, 9520599, 9520415, 9520560, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[519], &values);

    // Harmonica-E3
    let values: [i32; 7] = [9520631, 9526013, 9525776, 9525969, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[520], &values);

    // Bell Organ
    let values: [i32; 7] = [9526045, 9583421, 9530807, 9583415, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[521], &values);

    // Celeste-D#7
    let values: [i32; 7] = [9583453, 9593013, 9592634, 9593007, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[522], &values);

    // Celeste-D#6
    let values: [i32; 7] = [9593045, 9599302, 9599083, 9599296, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[523], &values);

    // Celeste-A4
    let values: [i32; 7] = [9599334, 9606676, 9606444, 9606670, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[524], &values);

    // Vibraphone-B6
    let values: [i32; 7] = [9606708, 9617449, 9617336, 9617396, 30000, 71, 0];
    sample_util::check(&sf.sample_headers[525], &values);

    // Vibraphone-C6
    let values: [i32; 7] = [9617481, 9621448, 9621370, 9621427, 30000, 72, 0];
    sample_util::check(&sf.sample_headers[526], &values);

    // Vibraphone-A#4
    let values: [i32; 7] = [9621480, 9626979, 9626744, 9626934, 30000, 58, 0];
    sample_util::check(&sf.sample_headers[527], &values);

    // Vibraphone-D4
    let values: [i32; 7] = [9627011, 9634060, 9633887, 9633988, 30000, 50, 0];
    sample_util::check(&sf.sample_headers[528], &values);

    // Vibraphone-F3
    let values: [i32; 7] = [9634092, 9640928, 9640721, 9640891, 30000, 41, 0];
    sample_util::check(&sf.sample_headers[529], &values);

    // Vibraphone-A#2
    let values: [i32; 7] = [9640960, 9652951, 9652643, 9652896, 30000, 34, 0];
    sample_util::check(&sf.sample_headers[530], &values);

    // Xylophone-C7
    let values: [i32; 7] = [9652983, 9654312, 9652987, 9654330, 20000, 60, 0];
    sample_util::check(&sf.sample_headers[531], &values);

    // Xylophone-E5
    let values: [i32; 7] = [9654344, 9664441, 9654344, 9664441, 20000, 60, 0];
    sample_util::check(&sf.sample_headers[532], &values);

    // Xylophone-G#4
    let values: [i32; 7] = [9664473, 9674341, 9664473, 9674340, 20000, 60, 0];
    sample_util::check(&sf.sample_headers[533], &values);

    // Xylophone-E4
    let values: [i32; 7] = [9674373, 9684168, 9674373, 9684168, 20000, 60, 0];
    sample_util::check(&sf.sample_headers[534], &values);

    // Clavinet-E6
    let values: [i32; 7] = [9684200, 9698228, 9698102, 9698201, 32000, 88, 0];
    sample_util::check(&sf.sample_headers[535], &values);

    // Clavinet-A5
    let values: [i32; 7] = [9698260, 9717668, 9717585, 9717622, 32000, 81, 0];
    sample_util::check(&sf.sample_headers[536], &values);

    // Clavinet-D5
    let values: [i32; 7] = [9717700, 9728736, 9728457, 9728623, 32000, 74, 0];
    sample_util::check(&sf.sample_headers[537], &values);

    // Clavinet-E4
    let values: [i32; 7] = [9728768, 9741804, 9741652, 9741751, 32000, 64, 0];
    sample_util::check(&sf.sample_headers[538], &values);

    // Clavinet-A3
    let values: [i32; 7] = [9741836, 9760248, 9760056, 9760204, 32000, 57, 0];
    sample_util::check(&sf.sample_headers[539], &values);

    // Clavinet-D3
    let values: [i32; 7] = [9760280, 9776116, 9775624, 9776069, 32000, 50, 0];
    sample_util::check(&sf.sample_headers[540], &values);

    // Clavinet-F#2
    let values: [i32; 7] = [9776148, 9799989, 9799539, 9799885, 32000, 42, 0];
    sample_util::check(&sf.sample_headers[541], &values);

    // Clavinet-B1
    let values: [i32; 7] = [9800021, 9822956, 9822375, 9822889, 32000, 35, 0];
    sample_util::check(&sf.sample_headers[542], &values);

    // Trumpet-B5
    let values: [i32; 7] = [9822988, 9830979, 9830918, 9830949, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[543], &values);

    // Trumpet-G5
    let values: [i32; 7] = [9831011, 9839519, 9839450, 9839489, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[544], &values);

    // Trumpet-F#6
    let values: [i32; 7] = [9839551, 9852790, 9852731, 9852773, 31137, 72, 0];
    sample_util::check(&sf.sample_headers[545], &values);

    // Trumpet-D5
    let values: [i32; 7] = [9852822, 9866222, 9866156, 9866209, 31209, 72, 0];
    sample_util::check(&sf.sample_headers[546], &values);

    // Trumpet-A4
    let values: [i32; 7] = [9866254, 9873570, 9873485, 9873556, 31137, 72, 0];
    sample_util::check(&sf.sample_headers[547], &values);

    // Trumpet-E4
    let values: [i32; 7] = [9873602, 9884535, 9884375, 9884470, 31245, 72, 0];
    sample_util::check(&sf.sample_headers[548], &values);

    // Trumpet-A#3
    let values: [i32; 7] = [9884567, 9895578, 9895375, 9895509, 31245, 72, 0];
    sample_util::check(&sf.sample_headers[549], &values);

    // Slap Bass 1-A4
    let values: [i32; 7] = [9895610, 9900846, 9899790, 9900826, 38000, 69, 0];
    sample_util::check(&sf.sample_headers[550], &values);

    // Slap Bass 1-C4
    let values: [i32; 7] = [9900878, 9910405, 9908933, 9910385, 38000, 60, 0];
    sample_util::check(&sf.sample_headers[551], &values);

    // Slap Bass 1-E3
    let values: [i32; 7] = [9910437, 9916533, 9916053, 9916514, 38000, 52, 0];
    sample_util::check(&sf.sample_headers[552], &values);

    // Slap Bass 1-G2
    let values: [i32; 7] = [9916565, 9923499, 9922703, 9923479, 38000, 43, 0];
    sample_util::check(&sf.sample_headers[553], &values);

    // Slap Bass 1-C2
    let values: [i32; 7] = [9923531, 9931116, 9929935, 9931097, 38000, 36, 0];
    sample_util::check(&sf.sample_headers[554], &values);

    // Jazz Guitar-E5
    let values: [i32; 7] = [9931148, 9937818, 9937768, 9937801, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[555], &values);

    // Jazz Guitar-C#5
    let values: [i32; 7] = [9937850, 9945974, 9945874, 9945953, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[556], &values);

    // Jazz Guitar-B4
    let values: [i32; 7] = [9946006, 9954068, 9953958, 9954047, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[557], &values);

    // Jazz Guitar-G4
    let values: [i32; 7] = [9954100, 9965937, 9965849, 9965905, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[558], &values);

    // Jazz Guitar-F#4
    let values: [i32; 7] = [9965969, 9976500, 9976350, 9976469, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[559], &values);

    // Jazz Guitar-D#4
    let values: [i32; 7] = [9976532, 9986492, 9986321, 9986462, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[560], &values);

    // Jazz Guitar-C4
    let values: [i32; 7] = [9986524, 9994245, 9994131, 9994215, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[561], &values);

    // Jazz Guitar-A#3
    let values: [i32; 7] = [9994277, 10004564, 10004326, 10004515, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[562], &values);

    // Jazz Guitar-G3
    let values: [i32; 7] = [10004596, 10016574, 10016281, 10016506, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[563], &values);

    // Jazz Guitar-F3
    let values: [i32; 7] = [10016606, 10027286, 10026979, 10027230, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[564], &values);

    // Jazz Guitar-D3
    let values: [i32; 7] = [10027318, 10037680, 10037330, 10037630, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[565], &values);

    // Jazz Guitar-C3
    let values: [i32; 7] = [10037712, 10046946, 10046717, 10046885, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[566], &values);

    // Jazz Guitar-A2
    let values: [i32; 7] = [10046978, 10055461, 10055132, 10055332, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[567], &values);

    // Jazz Guitar-F#2
    let values: [i32; 7] = [10055493, 10064842, 10064537, 10064775, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[568], &values);

    // Jazz Guitar-E2
    let values: [i32; 7] = [10064874, 10074322, 10074005, 10074272, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[569], &values);

    // Solo Vox-A#7
    let values: [i32; 7] = [10074354, 10076893, 10074392, 10076794, 11025, 60, 0];
    sample_util::check(&sf.sample_headers[570], &values);

    // Solo Vox-A#5
    let values: [i32; 7] = [10076925, 10082003, 10076949, 10081992, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[571], &values);

    // Solo Vox-G5
    let values: [i32; 7] = [10082035, 10086411, 10082043, 10086391, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[572], &values);

    // Solo Vox-G4
    let values: [i32; 7] = [10086443, 10090161, 10086485, 10090127, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[573], &values);

    // Solo Vox-C4
    let values: [i32; 7] = [10090193, 10093885, 10090207, 10093850, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[574], &values);

    // Solo Vox-G3
    let values: [i32; 7] = [10093917, 10100655, 10093992, 10100587, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[575], &values);

    // Synth Voice Aahs-B5
    let values: [i32; 7] = [10100687, 10122007, 10107360, 10121998, 28800, 83, 0];
    sample_util::check(&sf.sample_headers[576], &values);

    // Synth Voice Aahs-C#5
    let values: [i32; 7] = [10122039, 10158057, 10135924, 10158015, 28800, 73, 0];
    sample_util::check(&sf.sample_headers[577], &values);

    // Synth Voice Aahs-G4
    let values: [i32; 7] = [10158089, 10188404, 10170612, 10188356, 28800, 67, 0];
    sample_util::check(&sf.sample_headers[578], &values);

    // Synth Voice Aahs-C#4
    let values: [i32; 7] = [10188436, 10214994, 10202661, 10214963, 28800, 61, 0];
    sample_util::check(&sf.sample_headers[579], &values);

    // Synth Voice Aahs-A#3
    let values: [i32; 7] = [10215026, 10257340, 10222850, 10257280, 28800, 58, 0];
    sample_util::check(&sf.sample_headers[580], &values);

    // Synth Voice Aahs-G3
    let values: [i32; 7] = [10257372, 10298584, 10261963, 10298515, 28800, 55, 0];
    sample_util::check(&sf.sample_headers[581], &values);

    // Koto-E4
    let values: [i32; 7] = [10298616, 10331102, 10330890, 10331078, 31250, 64, 0];
    sample_util::check(&sf.sample_headers[582], &values);

    // Koto-A3
    let values: [i32; 7] = [10331134, 10364347, 10364011, 10364294, 31250, 57, 0];
    sample_util::check(&sf.sample_headers[583], &values);

    // Banjo-F5
    let values: [i32; 7] = [10364379, 10371101, 10371007, 10371067, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[584], &values);

    // Banjo-F4
    let values: [i32; 7] = [10371133, 10377319, 10377142, 10377261, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[585], &values);

    // Banjo-F3
    let values: [i32; 7] = [10377351, 10384904, 10384600, 10384838, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[586], &values);

    // Accordian-D2
    let values: [i32; 7] = [10384936, 10395538, 10395295, 10395530, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[587], &values);

    // Accordian-A#5
    let values: [i32; 7] = [10395570, 10403594, 10403568, 10403586, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[588], &values);

    // Accordian-F#5
    let values: [i32; 7] = [10403626, 10411704, 10411673, 10411696, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[589], &values);

    // Accordian-D5
    let values: [i32; 7] = [10411736, 10421352, 10421315, 10421344, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[590], &values);

    // Accordian-A#4
    let values: [i32; 7] = [10421384, 10425278, 10425159, 10425270, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[591], &values);

    // Accordian-F#4
    let values: [i32; 7] = [10425310, 10432482, 10432427, 10432474, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[592], &values);

    // Accordian-D4
    let values: [i32; 7] = [10432514, 10436321, 10436254, 10436313, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[593], &values);

    // Accordian-A#3
    let values: [i32; 7] = [10436353, 10445224, 10445142, 10445216, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[594], &values);

    // Accordian-F#3
    let values: [i32; 7] = [10445256, 10451860, 10451759, 10451852, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[595], &values);

    // Accordian-D3
    let values: [i32; 7] = [10451892, 10467755, 10467630, 10467748, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[596], &values);

    // Accordian-A#2
    let values: [i32; 7] = [10467787, 10481634, 10481478, 10481626, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[597], &values);

    // Accordian-F#2
    let values: [i32; 7] = [10481666, 10491881, 10491687, 10491874, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[598], &values);

    // Orchestra Hit-1
    let values: [i32; 7] = [10491913, 10504597, 10491917, 10504597, 23000, 60, 0];
    sample_util::check(&sf.sample_headers[599], &values);

    // Orchestra Hit-2
    let values: [i32; 7] = [10504629, 10525036, 10504630, 10525036, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[600], &values);

    // Timpani Soft
    let values: [i32; 7] = [10525068, 10550711, 10544986, 10550688, 11025, 50, 0];
    sample_util::check(&sf.sample_headers[601], &values);

    // Timpani Hard
    let values: [i32; 7] = [10550743, 10601474, 10550746, 10601467, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[602], &values);

    // Standard Tom 4
    let values: [i32; 7] = [10601506, 10681218, 10680265, 10681074, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[603], &values);

    // Standard Tom 3
    let values: [i32; 7] = [10681250, 10741234, 10740292, 10740902, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[604], &values);

    // Standard Tom 5
    let values: [i32; 7] = [10741266, 10815890, 10814644, 10815632, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[605], &values);

    // Standard Tom 2
    let values: [i32; 7] = [10815922, 10845490, 10843710, 10844868, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[606], &values);

    // Standard Tom 1
    let values: [i32; 7] = [10845522, 10869234, 10868143, 10868740, 32000, 60, 0];
    sample_util::check(&sf.sample_headers[607], &values);

    // Room Tom 2
    let values: [i32; 7] = [10869266, 10908423, 10869267, 10908423, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[608], &values);

    // Room Tom 1
    let values: [i32; 7] = [10908455, 10948508, 10908456, 10948508, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[609], &values);

    // Tambourine
    let values: [i32; 7] = [10948540, 10955240, 10948548, 10955232, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[610], &values);

    // Castanets
    let values: [i32; 7] = [10955272, 10956915, 10955280, 10956907, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[611], &values);

    // Wood Block Hard
    let values: [i32; 7] = [10956947, 10962290, 10956955, 10962282, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[612], &values);

    // Agogo
    let values: [i32; 7] = [10962322, 10968663, 10962330, 10968655, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[613], &values);

    // Filter Snap
    let values: [i32; 7] = [10968695, 10970500, 10968696, 10970500, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[614], &values);

    // Ride Cymbal
    let values: [i32; 7] = [10970532, 11091832, 10970540, 11091824, 28000, 51, 0];
    sample_util::check(&sf.sample_headers[615], &values);

    // Crash Cymbal
    let values: [i32; 7] = [11091864, 11198163, 11091872, 11198155, 28000, 57, 0];
    sample_util::check(&sf.sample_headers[616], &values);

    // Organ Fast-F#5
    let values: [i32; 7] = [11198195, 11202059, 11198773, 11202051, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[617], &values);

    // Organ Fast-C5
    let values: [i32; 7] = [11202091, 11205980, 11202699, 11205972, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[618], &values);

    // Organ Fast-F#4
    let values: [i32; 7] = [11206012, 11209956, 11206692, 11209950, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[619], &values);

    // Organ Fast-C4
    let values: [i32; 7] = [11209988, 11214218, 11210590, 11214211, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[620], &values);

    // Organ Fast-F#3
    let values: [i32; 7] = [11214250, 11218317, 11214858, 11218309, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[621], &values);

    // Organ Fast-C3
    let values: [i32; 7] = [11218349, 11222169, 11218962, 11222161, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[622], &values);

    // Organ Fast-F#2
    let values: [i32; 7] = [11222201, 11226094, 11222771, 11226087, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[623], &values);

    // Organ Fast-C2
    let values: [i32; 7] = [11226126, 11230156, 11226768, 11230148, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[624], &values);

    // Organ Fast-F#1
    let values: [i32; 7] = [11230188, 11234859, 11230796, 11234851, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[625], &values);

    // Organ Perc.-F#5
    let values: [i32; 7] = [11234891, 11246459, 11234898, 11246457, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[626], &values);

    // Organ Perc.-A4
    let values: [i32; 7] = [11246491, 11259258, 11246498, 11259256, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[627], &values);

    // Organ Perc.-C4
    let values: [i32; 7] = [11259290, 11271943, 11259297, 11271941, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[628], &values);

    // Organ Perc.-D#3
    let values: [i32; 7] = [11271975, 11284256, 11271982, 11284254, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[629], &values);

    // Organ Perc.-F#2
    let values: [i32; 7] = [11284288, 11296830, 11284295, 11296828, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[630], &values);

    // Organ Perc.-A1
    let values: [i32; 7] = [11296862, 11310083, 11296869, 11310081, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[631], &values);

    // Organ Slow-C4
    let values: [i32; 7] = [11310115, 11315717, 11310355, 11315708, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[632], &values);

    // Organ Slow-D#3
    let values: [i32; 7] = [11315749, 11321126, 11315998, 11321118, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[633], &values);

    // Organ Slow-F#2
    let values: [i32; 7] = [11321158, 11326135, 11321417, 11326127, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[634], &values);

    // Trombone-C4
    let values: [i32; 7] = [11326167, 11334817, 11334725, 11334809, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[635], &values);

    // Trombone-G3
    let values: [i32; 7] = [11334849, 11344872, 11344752, 11344864, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[636], &values);

    // Trombone-D3
    let values: [i32; 7] = [11344904, 11353728, 11353570, 11353720, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[637], &values);

    // Trombone-A2
    let values: [i32; 7] = [11353760, 11364018, 11363810, 11364010, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[638], &values);

    // Trombone-E2
    let values: [i32; 7] = [11364050, 11374751, 11374439, 11374706, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[639], &values);

    // Trombone-B1
    let values: [i32; 7] = [11374783, 11386270, 11385906, 11386262, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[640], &values);

    // Trombone-C#1
    let values: [i32; 7] = [11386302, 11402195, 11401552, 11402187, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[641], &values);

    // Shakuhachi
    let values: [i32; 7] = [11402227, 11467905, 11444766, 11466897, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[642], &values);

    // Whistle-D5
    let values: [i32; 7] = [11467937, 11473525, 11470718, 11473515, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[643], &values);

    // Whistle-A5
    let values: [i32; 7] = [11473557, 11477722, 11475272, 11477711, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[644], &values);

    // Whistle-A4
    let values: [i32; 7] = [11477754, 11487024, 11482097, 11486983, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[645], &values);

    // Whistle-B4
    let values: [i32; 7] = [11487056, 11495610, 11491062, 11495601, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[646], &values);

    // Whistle-E4
    let values: [i32; 7] = [11495642, 11507857, 11502673, 11507845, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[647], &values);

    // Chiff Flute
    let values: [i32; 7] = [11507889, 11524033, 11515502, 11523994, 22050, 62, 0];
    sample_util::check(&sf.sample_headers[648], &values);

    // Bottle Blow
    let values: [i32; 7] = [11524065, 11555681, 11541847, 11555622, 22050, 66, 0];
    sample_util::check(&sf.sample_headers[649], &values);

    // Pan Flute_1
    let values: [i32; 7] = [11555713, 11560126, 11555721, 11560126, 30000, 81, 0];
    sample_util::check(&sf.sample_headers[650], &values);

    // Pan Flute_2
    let values: [i32; 7] = [11560158, 11592513, 11578868, 11592491, 30000, 81, 0];
    sample_util::check(&sf.sample_headers[651], &values);

    // Bassoon-C2
    let values: [i32; 7] = [11592545, 11598518, 11597985, 11598486, 33000, 36, 0];
    sample_util::check(&sf.sample_headers[652], &values);

    // Bassoon-D2
    let values: [i32; 7] = [11598550, 11605746, 11605237, 11605686, 33000, 38, 0];
    sample_util::check(&sf.sample_headers[653], &values);

    // Bassoon-E2
    let values: [i32; 7] = [11605778, 11611807, 11611371, 11611772, 33000, 40, 0];
    sample_util::check(&sf.sample_headers[654], &values);

    // Bassoon-C3
    let values: [i32; 7] = [11611839, 11618822, 11618535, 11618790, 33000, 48, 0];
    sample_util::check(&sf.sample_headers[655], &values);

    // Bassoon-D3
    let values: [i32; 7] = [11618854, 11628241, 11627976, 11628202, 33000, 50, 0];
    sample_util::check(&sf.sample_headers[656], &values);

    // Bassoon-E3
    let values: [i32; 7] = [11628273, 11637623, 11637389, 11637591, 33000, 52, 0];
    sample_util::check(&sf.sample_headers[657], &values);

    // Bassoon-A#3
    let values: [i32; 7] = [11637655, 11643879, 11643693, 11643835, 33000, 58, 0];
    sample_util::check(&sf.sample_headers[658], &values);

    // Bassoon-C4
    let values: [i32; 7] = [11643911, 11649856, 11649691, 11649819, 33000, 60, 0];
    sample_util::check(&sf.sample_headers[659], &values);

    // Bassoon-D4
    let values: [i32; 7] = [11649888, 11656030, 11655879, 11655994, 33000, 62, 0];
    sample_util::check(&sf.sample_headers[660], &values);

    // Bassoon-F#4
    let values: [i32; 7] = [11656062, 11662031, 11661909, 11661999, 33000, 66, 0];
    sample_util::check(&sf.sample_headers[661], &values);

    // Bassoon-G#4
    let values: [i32; 7] = [11662063, 11667853, 11667738, 11667818, 33000, 68, 0];
    sample_util::check(&sf.sample_headers[662], &values);

    // Bassoon-C5
    let values: [i32; 7] = [11667885, 11674111, 11674009, 11674072, 33000, 72, 0];
    sample_util::check(&sf.sample_headers[663], &values);

    // Viola-D6
    let values: [i32; 7] = [11674143, 11697645, 11693328, 11697620, 22050, 86, 0];
    sample_util::check(&sf.sample_headers[664], &values);

    // Viola-B5
    let values: [i32; 7] = [11697677, 11719782, 11715707, 11719757, 22050, 83, 0];
    sample_util::check(&sf.sample_headers[665], &values);

    // Viola-A#5
    let values: [i32; 7] = [11719814, 11741920, 11738380, 11741895, 22050, 82, 0];
    sample_util::check(&sf.sample_headers[666], &values);

    // Viola-F5
    let values: [i32; 7] = [11741952, 11763368, 11758799, 11763343, 22050, 77, 0];
    sample_util::check(&sf.sample_headers[667], &values);

    // Viola-D5
    let values: [i32; 7] = [11763400, 11785298, 11781895, 11785273, 22050, 74, 0];
    sample_util::check(&sf.sample_headers[668], &values);

    // Viola-C5
    let values: [i32; 7] = [11785330, 11807506, 11804067, 11807348, 22050, 72, 0];
    sample_util::check(&sf.sample_headers[669], &values);

    // Viola-G#4
    let values: [i32; 7] = [11807538, 11829344, 11825390, 11829319, 22050, 68, 0];
    sample_util::check(&sf.sample_headers[670], &values);

    // Viola-E4
    let values: [i32; 7] = [11829376, 11850889, 11846840, 11850865, 22050, 64, 0];
    sample_util::check(&sf.sample_headers[671], &values);

    // Viola-C4
    let values: [i32; 7] = [11850921, 11872513, 11868619, 11872489, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[672], &values);

    // Viola-A3
    let values: [i32; 7] = [11872545, 11893824, 11890126, 11893803, 22050, 57, 0];
    sample_util::check(&sf.sample_headers[673], &values);

    // Viola-F#3
    let values: [i32; 7] = [11893856, 11915709, 11912111, 11915684, 22050, 54, 0];
    sample_util::check(&sf.sample_headers[674], &values);

    // Viola-D#3
    let values: [i32; 7] = [11915741, 11935759, 11931888, 11935733, 22050, 51, 0];
    sample_util::check(&sf.sample_headers[675], &values);

    // Viola-C#3
    let values: [i32; 7] = [11935791, 11957591, 11953426, 11957567, 22050, 49, 0];
    sample_util::check(&sf.sample_headers[676], &values);

    // Fretless Bass-G#4
    let values: [i32; 7] = [11957623, 11969234, 11969088, 11969194, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[677], &values);

    // Fretless Bass-G#3
    let values: [i32; 7] = [11969266, 11984625, 11984377, 11984590, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[678], &values);

    // Fretless Bass-D3
    let values: [i32; 7] = [11984657, 11999026, 11998843, 11998993, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[679], &values);

    // Fretless Bass-G#2
    let values: [i32; 7] = [11999058, 12014015, 12013513, 12013938, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[680], &values);

    // Hammond Organ-11
    let values: [i32; 7] = [12014047, 12014856, 12014792, 12014846, 6000, 47, 0];
    sample_util::check(&sf.sample_headers[681], &values);

    // Hammond Organ-10
    let values: [i32; 7] = [12014888, 12016374, 12016255, 12016353, 11025, 47, 0];
    sample_util::check(&sf.sample_headers[682], &values);

    // Hammond Organ-9
    let values: [i32; 7] = [12016406, 12018901, 12018713, 12018878, 22050, 47, 0];
    sample_util::check(&sf.sample_headers[683], &values);

    // Hammond Organ-8
    let values: [i32; 7] = [12018933, 12021351, 12021155, 12021315, 22050, 47, 0];
    sample_util::check(&sf.sample_headers[684], &values);

    // Hammond Organ-7
    let values: [i32; 7] = [12021383, 12023593, 12023429, 12023575, 22050, 47, 0];
    sample_util::check(&sf.sample_headers[685], &values);

    // Hammond Organ-6
    let values: [i32; 7] = [12023625, 12025689, 12025366, 12025657, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[686], &values);

    // Hammond Organ-5
    let values: [i32; 7] = [12025721, 12027279, 12026995, 12027249, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[687], &values);

    // Hammond Organ-4
    let values: [i32; 7] = [12027311, 12028879, 12028559, 12028809, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[688], &values);

    // Hammond Organ-3
    let values: [i32; 7] = [12028911, 12030586, 12030275, 12030508, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[689], &values);

    // Hammond Organ-2
    let values: [i32; 7] = [12030618, 12031914, 12031683, 12031897, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[690], &values);

    // Hammond Organ-1
    let values: [i32; 7] = [12031946, 12033494, 12033099, 12033292, 22050, 35, 0];
    sample_util::check(&sf.sample_headers[691], &values);

    // Synth Bell-3
    let values: [i32; 7] = [12033526, 12045962, 12033691, 12045890, 24000, 60, 0];
    sample_util::check(&sf.sample_headers[692], &values);

    // Synth Bell-1
    let values: [i32; 7] = [12045994, 12070960, 12046125, 12070854, 48000, 60, 0];
    sample_util::check(&sf.sample_headers[693], &values);

    // Synth Bell-2
    let values: [i32; 7] = [12070992, 12095863, 12071057, 12095781, 48000, 60, 0];
    sample_util::check(&sf.sample_headers[694], &values);

    // Stacked Sawtooth-C6
    let values: [i32; 7] = [12095895, 12129024, 12097383, 12128838, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[695], &values);

    // Stacked Sawtooth-C4
    let values: [i32; 7] = [12129056, 12182708, 12134120, 12182165, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[696], &values);

    // Distortion Guitar-D3
    let values: [i32; 7] = [12182740, 12211871, 12211292, 12211633, 24999, 50, 0];
    sample_util::check(&sf.sample_headers[697], &values);

    // Distortion Guitar-A2
    let values: [i32; 7] = [12211903, 12232983, 12232697, 12232880, 19998, 45, 0];
    sample_util::check(&sf.sample_headers[698], &values);

    // Steel Guitar-E6
    let values: [i32; 7] = [12233015, 12239571, 12238897, 12239555, 24000, 88, 0];
    sample_util::check(&sf.sample_headers[699], &values);

    // Steel Guitar-C#6
    let values: [i32; 7] = [12239603, 12253819, 12250314, 12253791, 32000, 85, 0];
    sample_util::check(&sf.sample_headers[700], &values);

    // Steel Guitar-A#5
    let values: [i32; 7] = [12253851, 12266962, 12263915, 12266939, 32000, 82, 0];
    sample_util::check(&sf.sample_headers[701], &values);

    // Steel Guitar-G5
    let values: [i32; 7] = [12266994, 12284963, 12280938, 12284941, 32000, 79, 0];
    sample_util::check(&sf.sample_headers[702], &values);

    // Steel Guitar-E5
    let values: [i32; 7] = [12284995, 12306971, 12299193, 12306948, 32000, 76, 0];
    sample_util::check(&sf.sample_headers[703], &values);

    // Steel Guitar-B4
    let values: [i32; 7] = [12307003, 12338394, 12327109, 12338372, 32000, 71, 0];
    sample_util::check(&sf.sample_headers[704], &values);

    // Steel Guitar-G4
    let values: [i32; 7] = [12338426, 12364244, 12360145, 12364221, 32000, 67, 0];
    sample_util::check(&sf.sample_headers[705], &values);

    // Steel Guitar-D4
    let values: [i32; 7] = [12364276, 12381433, 12378798, 12381413, 24000, 62, 0];
    sample_util::check(&sf.sample_headers[706], &values);

    // Steel Guitar-A3
    let values: [i32; 7] = [12381465, 12403584, 12397707, 12403567, 24000, 57, 0];
    sample_util::check(&sf.sample_headers[707], &values);

    // Steel Guitar-E3
    let values: [i32; 7] = [12403616, 12430463, 12422911, 12430446, 24000, 52, 0];
    sample_util::check(&sf.sample_headers[708], &values);

    // Sitar-G5
    let values: [i32; 7] = [12430495, 12457209, 12430503, 12457201, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[709], &values);

    // Sitar-G4
    let values: [i32; 7] = [12457241, 12501550, 12457249, 12501542, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[710], &values);

    // Sitar-G3
    let values: [i32; 7] = [12501582, 12545891, 12501590, 12545883, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[711], &values);

    // Sitar-G2
    let values: [i32; 7] = [12545923, 12585398, 12545931, 12585390, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[712], &values);

    // Muted Guitar-E6
    let values: [i32; 7] = [12585430, 12586682, 12586606, 12586654, 8000, 60, 0];
    sample_util::check(&sf.sample_headers[713], &values);

    // Muted Guitar-E4
    let values: [i32; 7] = [12586714, 12590163, 12589890, 12589957, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[714], &values);

    // Muted Guitar-D#4
    let values: [i32; 7] = [12590195, 12593296, 12592553, 12592623, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[715], &values);

    // Muted Guitar-B3
    let values: [i32; 7] = [12593328, 12596105, 12595581, 12595669, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[716], &values);

    // Muted Guitar-B2
    let values: [i32; 7] = [12596137, 12610976, 12610364, 12610543, 22050, 60, 0];
    sample_util::check(&sf.sample_headers[717], &values);

    // Nylon Guitar-C4
    let values: [i32; 7] = [12611008, 12627834, 12627616, 12627784, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[718], &values);

    // Nylon Guitar-B3
    let values: [i32; 7] = [12627866, 12647133, 12646904, 12647083, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[719], &values);

    // Nylon Guitar-G#3
    let values: [i32; 7] = [12647165, 12659942, 12659681, 12659892, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[720], &values);

    // Nylon Guitar-G3
    let values: [i32; 7] = [12659974, 12674815, 12674653, 12674765, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[721], &values);

    // Nylon Guitar-D#3
    let values: [i32; 7] = [12674847, 12700694, 12700361, 12700644, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[722], &values);

    // Nylon Guitar-D3
    let values: [i32; 7] = [12700726, 12734241, 12734041, 12734191, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[723], &values);

    // Nylon Guitar-A#2
    let values: [i32; 7] = [12734273, 12757038, 12756825, 12757015, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[724], &values);

    // Nylon Guitar-F2
    let values: [i32; 7] = [12757070, 12780719, 12780416, 12780669, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[725], &values);

    // Applause
    let values: [i32; 7] = [12780751, 12870249, 12836241, 12870249, 25000, 60, 0];
    sample_util::check(&sf.sample_headers[726], &values);

    // Bagpipe Chant
    let values: [i32; 7] = [12870281, 12874399, 12874134, 12874175, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[727], &values);

    // Bagpipe Drone
    let values: [i32; 7] = [12874431, 12903111, 12874621, 12903105, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[728], &values);

    // Clarinet-A#5
    let values: [i32; 7] = [12903143, 12907400, 12905686, 12907368, 16801, 82, 0];
    sample_util::check(&sf.sample_headers[729], &values);

    // Clarinet-E5
    let values: [i32; 7] = [12907432, 12911680, 12909974, 12911648, 16798, 76, 0];
    sample_util::check(&sf.sample_headers[730], &values);

    // Clarinet-A#4
    let values: [i32; 7] = [12911712, 12915975, 12914255, 12915943, 16803, 70, 0];
    sample_util::check(&sf.sample_headers[731], &values);

    // Clarinet-E4
    let values: [i32; 7] = [12916007, 12920246, 12918550, 12920214, 16803, 64, 0];
    sample_util::check(&sf.sample_headers[732], &values);

    // Clarinet-A#3
    let values: [i32; 7] = [12920278, 12924563, 12922821, 12924531, 16802, 58, 0];
    sample_util::check(&sf.sample_headers[733], &values);

    // Clarinet-F3
    let values: [i32; 7] = [12924595, 12928888, 12927137, 12928856, 16796, 53, 0];
    sample_util::check(&sf.sample_headers[734], &values);

    // Recorder-A#5
    let values: [i32; 7] = [12928920, 12938280, 12928928, 12937421, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[735], &values);

    // Recorder-A5
    let values: [i32; 7] = [12938312, 12946336, 12943858, 12946328, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[736], &values);

    // Recorder-D5
    let values: [i32; 7] = [12946368, 12959072, 12950728, 12959012, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[737], &values);

    // Recorder-C#5
    let values: [i32; 7] = [12959104, 12968976, 12965433, 12968944, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[738], &values);

    // Recorder-B4
    let values: [i32; 7] = [12969008, 12991696, 12969016, 12991688, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[739], &values);

    // Recorder-A4
    let values: [i32; 7] = [12991728, 13002352, 12991736, 13002345, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[740], &values);

    // Recorder-D3
    let values: [i32; 7] = [13002384, 13041680, 13026812, 13041512, 45000, 60, 0];
    sample_util::check(&sf.sample_headers[741], &values);

    // Tenor Sax-Eb4
    let values: [i32; 7] = [13041712, 13047363, 13047285, 13047355, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[742], &values);

    // Tenor Sax-C4
    let values: [i32; 7] = [13047395, 13053684, 13053592, 13053676, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[743], &values);

    // Tenor Sax-F3
    let values: [i32; 7] = [13053716, 13062796, 13062662, 13062788, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[744], &values);

    // Tenor Sax-D3
    let values: [i32; 7] = [13062828, 13069726, 13069568, 13069718, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[745], &values);

    // Tenor Sax-Bb2
    let values: [i32; 7] = [13069758, 13072842, 13072645, 13072834, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[746], &values);

    // Tenor Sax-G2
    let values: [i32; 7] = [13072874, 13083396, 13083164, 13083388, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[747], &values);

    // Tenor Sax-E2
    let values: [i32; 7] = [13083428, 13093675, 13093400, 13093667, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[748], &values);

    // Tenor Sax-C2
    let values: [i32; 7] = [13093707, 13099171, 13098922, 13099161, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[749], &values);

    // Soprano Sax-C4
    let values: [i32; 7] = [13099203, 13115935, 13111060, 13115927, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[750], &values);

    // Soprano Sax-G#3
    let values: [i32; 7] = [13115967, 13133726, 13128618, 13133718, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[751], &values);

    // Soprano Sax-E3
    let values: [i32; 7] = [13133758, 13151332, 13146031, 13151324, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[752], &values);

    // Soprano Sax-C3
    let values: [i32; 7] = [13151364, 13168459, 13163185, 13168451, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[753], &values);

    // Soprano Sax-G#2
    let values: [i32; 7] = [13168491, 13185541, 13180424, 13185533, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[754], &values);

    // Soprano Sax-E2
    let values: [i32; 7] = [13185573, 13203274, 13197598, 13203266, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[755], &values);

    // Soprano Sax-C2
    let values: [i32; 7] = [13203306, 13220368, 13215065, 13220360, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[756], &values);

    // Soprano Sax-G#1
    let values: [i32; 7] = [13220400, 13237859, 13232395, 13237851, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[757], &values);

    // Piccolo-D3
    let values: [i32; 7] = [13237891, 13251878, 13246344, 13251873, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[758], &values);

    // Piccolo-A#2
    let values: [i32; 7] = [13251910, 13266314, 13260811, 13266309, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[759], &values);

    // Piccolo-F#2
    let values: [i32; 7] = [13266346, 13280615, 13275362, 13280610, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[760], &values);

    // Piccolo-D2
    let values: [i32; 7] = [13280647, 13295437, 13290224, 13295432, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[761], &values);

    // Piccolo-A#1
    let values: [i32; 7] = [13295469, 13315004, 13310015, 13314999, 27778, 60, 0];
    sample_util::check(&sf.sample_headers[762], &values);

    // Tuba-A#1
    let values: [i32; 7] = [13315036, 13320900, 13320723, 13320895, 20046, 60, 0];
    sample_util::check(&sf.sample_headers[763], &values);

    // Tuba-F#1
    let values: [i32; 7] = [13320932, 13325663, 13325442, 13325658, 19980, 60, 0];
    sample_util::check(&sf.sample_headers[764], &values);

    // Tuba-D1
    let values: [i32; 7] = [13325695, 13331391, 13331113, 13331386, 20043, 60, 0];
    sample_util::check(&sf.sample_headers[765], &values);

    // Tuba-F#0
    let values: [i32; 7] = [13331423, 13336486, 13336049, 13336481, 19980, 60, 0];
    sample_util::check(&sf.sample_headers[766], &values);

    // Muted Trumpet-F#5
    let values: [i32; 7] = [13336518, 13350361, 13347809, 13350329, 31064, 72, 0];
    sample_util::check(&sf.sample_headers[767], &values);

    // Muted Trumpet-D5
    let values: [i32; 7] = [13350393, 13364471, 13361891, 13364439, 30601, 72, 0];
    sample_util::check(&sf.sample_headers[768], &values);

    // Muted Trumpet-A#4
    let values: [i32; 7] = [13364503, 13385017, 13382450, 13384985, 30460, 72, 0];
    sample_util::check(&sf.sample_headers[769], &values);

    // Muted Trumpet-D4
    let values: [i32; 7] = [13385049, 13399302, 13396670, 13399270, 30637, 72, 0];
    sample_util::check(&sf.sample_headers[770], &values);

    // Pizz. Strings-F5
    let values: [i32; 7] = [13399334, 13411486, 13411422, 13411454, 22505, 72, 0];
    sample_util::check(&sf.sample_headers[771], &values);

    // Pizz. Strings-F#4
    let values: [i32; 7] = [13411518, 13427190, 13427126, 13427158, 22505, 72, 0];
    sample_util::check(&sf.sample_headers[772], &values);

    // Pizz. Strings-G#3
    let values: [i32; 7] = [13427222, 13446044, 13445980, 13446012, 22505, 72, 0];
    sample_util::check(&sf.sample_headers[773], &values);

    // Pizz. Strings-A#2
    let values: [i32; 7] = [13446076, 13466815, 13466751, 13466783, 22505, 72, 0];
    sample_util::check(&sf.sample_headers[774], &values);

    // Pipe Organ-G#6
    let values: [i32; 7] = [13466847, 13525923, 13478065, 13525861, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[775], &values);

    // Pipe Organ-G#5
    let values: [i32; 7] = [13525955, 13585115, 13537185, 13585053, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[776], &values);

    // Pipe Organ-G#4
    let values: [i32; 7] = [13585147, 13644437, 13596408, 13644375, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[777], &values);

    // Pipe Organ-G#3
    let values: [i32; 7] = [13644469, 13703611, 13655710, 13703549, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[778], &values);

    // Pipe Organ-G#2
    let values: [i32; 7] = [13703643, 13743291, 13713976, 13743229, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[779], &values);

    // Pipe Organ-A#1
    let values: [i32; 7] = [13743323, 13802435, 13758713, 13802373, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[780], &values);

    // Brass Section-C6
    let values: [i32; 7] = [13802467, 13864521, 13825047, 13864513, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[781], &values);

    // Brass Section-G5
    let values: [i32; 7] = [13864553, 13931730, 13884038, 13931723, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[782], &values);

    // Brass Section-E5
    let values: [i32; 7] = [13931762, 14009462, 13959390, 14009455, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[783], &values);

    // Brass Section-C5
    let values: [i32; 7] = [14009494, 14085067, 14021964, 14085059, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[784], &values);

    // Brass Section-G4
    let values: [i32; 7] = [14085099, 14161087, 14105197, 14161079, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[785], &values);

    // Brass Section-C4
    let values: [i32; 7] = [14161119, 14261551, 14187143, 14261543, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[786], &values);

    // Pick Bass-G1
    let values: [i32; 7] = [14261583, 14279564, 14279330, 14279534, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[787], &values);

    // Pick Bass-D1
    let values: [i32; 7] = [14279596, 14298109, 14297818, 14298091, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[788], &values);

    // Pick Bass-A0
    let values: [i32; 7] = [14298141, 14313649, 14313292, 14313635, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[789], &values);

    // Pick Bass-E0
    let values: [i32; 7] = [14313681, 14334404, 14333721, 14334206, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[790], &values);

    // Finger Bass-G1
    let values: [i32; 7] = [14334436, 14344697, 14344585, 14344687, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[791], &values);

    // Finger Bass-D1
    let values: [i32; 7] = [14344729, 14355102, 14354958, 14355094, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[792], &values);

    // Finger Bass-A0
    let values: [i32; 7] = [14355134, 14364952, 14364751, 14364921, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[793], &values);

    // Finger Bass-E0
    let values: [i32; 7] = [14364984, 14375533, 14375284, 14375524, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[794], &values);

    // Dulcimer-E6
    let values: [i32; 7] = [14375565, 14426230, 14426188, 14426222, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[795], &values);

    // Dulcimer-B5
    let values: [i32; 7] = [14426262, 14460274, 14460231, 14460266, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[796], &values);

    // Dulcimer-F#5
    let values: [i32; 7] = [14460306, 14490125, 14490071, 14490117, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[797], &values);

    // Dulcimer-C5
    let values: [i32; 7] = [14490157, 14533964, 14533896, 14533957, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[798], &values);

    // Dulcimer-F#4
    let values: [i32; 7] = [14533996, 14564160, 14564063, 14564152, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[799], &values);

    // Dulcimer-C#4
    let values: [i32; 7] = [14564192, 14609443, 14609323, 14609435, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[800], &values);

    // Tubular Bells-A6
    let values: [i32; 7] = [14609475, 14642311, 14633469, 14642303, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[801], &values);

    // Tubular Bells-A5
    let values: [i32; 7] = [14642343, 14708017, 14690339, 14708009, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[802], &values);

    // Tubular Bells-D5
    let values: [i32; 7] = [14708049, 14769627, 14751341, 14769619, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[803], &values);

    // Tubular Bells-G4
    let values: [i32; 7] = [14769659, 14826757, 14810494, 14826749, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[804], &values);

    // Tubular Bells-C4
    let values: [i32; 7] = [14826789, 14897176, 14874180, 14897168, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[805], &values);

    // Glockenspiel-G#6
    let values: [i32; 7] = [14897208, 14898498, 14898343, 14898493, 31250, 60, 0];
    sample_util::check(&sf.sample_headers[806], &values);

    // Oboe-C5
    let values: [i32; 7] = [14898530, 14921419, 14916127, 14921414, 25117, 60, 0];
    sample_util::check(&sf.sample_headers[807], &values);

    // Oboe-A4
    let values: [i32; 7] = [14921451, 14947785, 14943020, 14947780, 24641, 60, 0];
    sample_util::check(&sf.sample_headers[808], &values);

    // Oboe-E4
    let values: [i32; 7] = [14947817, 14976844, 14972192, 14976839, 25052, 60, 0];
    sample_util::check(&sf.sample_headers[809], &values);

    // Oboe-A#3
    let values: [i32; 7] = [14976876, 14998711, 14993454, 14998706, 25173, 60, 0];
    sample_util::check(&sf.sample_headers[810], &values);

    // Oboe-F#3
    let values: [i32; 7] = [14998743, 15028810, 15023696, 15028805, 25160, 60, 0];
    sample_util::check(&sf.sample_headers[811], &values);

    // Oboe-C#3
    let values: [i32; 7] = [15028842, 15059399, 15053890, 15059394, 24947, 60, 0];
    sample_util::check(&sf.sample_headers[812], &values);

    // Synth Strings 2-C5
    let values: [i32; 7] = [15059431, 15082366, 15059436, 15082358, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[813], &values);

    // Synth Strings 2-C4
    let values: [i32; 7] = [15082398, 15108048, 15082403, 15108040, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[814], &values);

    // Synth Strings 2-C3
    let values: [i32; 7] = [15108080, 15127904, 15108085, 15127896, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[815], &values);

    // Synth Strings 2-C2
    let values: [i32; 7] = [15127936, 15147045, 15127941, 15147036, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[816], &values);

    // Clean Guitar-D3
    let values: [i32; 7] = [15147077, 15182667, 15181279, 15182635, 25000, 50, 0];
    sample_util::check(&sf.sample_headers[817], &values);

    // Clean Guitar-A2
    let values: [i32; 7] = [15182699, 15208323, 15206934, 15208291, 25000, 45, 0];
    sample_util::check(&sf.sample_headers[818], &values);

    // Clean Guitar-E2
    let values: [i32; 7] = [15208355, 15229047, 15227504, 15229015, 25000, 40, 0];
    sample_util::check(&sf.sample_headers[819], &values);

    // Baritone Sax-D4
    let values: [i32; 7] = [15229079, 15238149, 15233491, 15238023, 44100, 82, 0];
    sample_util::check(&sf.sample_headers[820], &values);

    // Baritone Sax-A#3
    let values: [i32; 7] = [15238181, 15244430, 15241890, 15244394, 44100, 78, 0];
    sample_util::check(&sf.sample_headers[821], &values);

    // Baritone Sax-F3
    let values: [i32; 7] = [15244462, 15256117, 15249644, 15255924, 44100, 74, 0];
    sample_util::check(&sf.sample_headers[822], &values);

    // Baritone Sax-D3
    let values: [i32; 7] = [15256149, 15265023, 15263907, 15264991, 44100, 70, 0];
    sample_util::check(&sf.sample_headers[823], &values);

    // Baritone Sax-A#2
    let values: [i32; 7] = [15265055, 15275840, 15272550, 15273027, 44100, 66, 0];
    sample_util::check(&sf.sample_headers[824], &values);

    // Baritone Sax-F#2
    let values: [i32; 7] = [15275872, 15289778, 15288283, 15289746, 44100, 62, 0];
    sample_util::check(&sf.sample_headers[825], &values);

    // Baritone Sax-D2
    let values: [i32; 7] = [15289810, 15302365, 15302301, 15302333, 44100, 58, 0];
    sample_util::check(&sf.sample_headers[826], &values);

    // Baritone Sax-A#1
    let values: [i32; 7] = [15302397, 15312500, 15310970, 15312468, 44100, 54, 0];
    sample_util::check(&sf.sample_headers[827], &values);

    // Baritone Sax-F#1
    let values: [i32; 7] = [15312532, 15326228, 15326164, 15326196, 44100, 50, 0];
    sample_util::check(&sf.sample_headers[828], &values);

    // Techno Bass
    let values: [i32; 7] = [15326260, 15332112, 15326261, 15332112, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[829], &values);

    // Glockenspiel-D6
    let values: [i32; 7] = [15332144, 15333790, 15333687, 15333782, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[830], &values);

    // Guitar Harmonics-Gb2
    let values: [i32; 7] = [15333822, 15341757, 15341561, 15341749, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[831], &values);

    // Guitar Harmonics-B2
    let values: [i32; 7] = [15341789, 15347666, 15347235, 15347658, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[832], &values);

    // Guitar Harmonics-Eb3
    let values: [i32; 7] = [15347698, 15348927, 15348808, 15348919, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[833], &values);

    // Guitar Harmonics-Ab3
    let values: [i32; 7] = [15348959, 15349813, 15349638, 15349805, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[834], &values);

    // Guitar Harmonics-C5
    let values: [i32; 7] = [15349845, 15350182, 15350141, 15350174, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[835], &values);

    // Guitar Harmonics-C6
    let values: [i32; 7] = [15350214, 15350356, 15350332, 15350348, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[836], &values);

    // Pick Bass-C6
    let values: [i32; 7] = [15350388, 15350527, 15350508, 15350519, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[837], &values);

    // Voice Oohs-G2
    let values: [i32; 7] = [15350559, 15366555, 15358837, 15366547, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[838], &values);

    // Voice Oohs-B2
    let values: [i32; 7] = [15366587, 15378109, 15371321, 15378101, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[839], &values);

    // Voice Oohs-Eb3
    let values: [i32; 7] = [15378141, 15391450, 15383802, 15391442, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[840], &values);

    // Voice Oohs-G3
    let values: [i32; 7] = [15391482, 15403739, 15396078, 15403731, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[841], &values);

    // Voice Oohs-B4
    let values: [i32; 7] = [15403771, 15415815, 15409095, 15415807, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[842], &values);

    // Voice Oohs-Eb4
    let values: [i32; 7] = [15415847, 15428630, 15420250, 15428622, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[843], &values);

    // Voice Oohs-C6
    let values: [i32; 7] = [15428662, 15432462, 15429978, 15432454, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[844], &values);

    // Siku-C6
    let values: [i32; 7] = [15432494, 15432845, 15432826, 15432837, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[845], &values);

    // Goblin Loop-Eb3
    let values: [i32; 7] = [15432877, 15438056, 15437533, 15438048, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[846], &values);

    // Goblin Loop-Eb4
    let values: [i32; 7] = [15438088, 15440203, 15440158, 15440195, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[847], &values);

    // Goblin Loop-Eb6
    let values: [i32; 7] = [15440235, 15440501, 15440483, 15440493, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[848], &values);

    // Kalimba-C3
    let values: [i32; 7] = [15440533, 15449923, 15449808, 15449915, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[849], &values);

    // Kalimba-C5
    let values: [i32; 7] = [15449955, 15452238, 15452204, 15452230, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[850], &values);

    // Kalimba-C6
    let values: [i32; 7] = [15452270, 15452697, 15452676, 15452689, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[851], &values);

    // Shenai-D6
    let values: [i32; 7] = [15452729, 15452972, 15452954, 15452964, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[852], &values);

    // Shenai-C3
    let values: [i32; 7] = [15453004, 15453626, 15453524, 15453618, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[853], &values);

    // Shenai-D3
    let values: [i32; 7] = [15453658, 15454492, 15454399, 15454484, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[854], &values);

    // Shenai-F#3
    let values: [i32; 7] = [15454524, 15455298, 15455222, 15455290, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[855], &values);

    // Shenai-A#3
    let values: [i32; 7] = [15455330, 15456320, 15456257, 15456312, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[856], &values);

    // Shenai-D4
    let values: [i32; 7] = [15456352, 15457410, 15457358, 15457402, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[857], &values);

    // Carillon-C6
    let values: [i32; 7] = [15457442, 15467505, 15466306, 15467497, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[858], &values);

    // Steel Drums-D5
    let values: [i32; 7] = [15467537, 15469594, 15469361, 15469586, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[859], &values);

    // Wood Block Med
    let values: [i32; 7] = [15469626, 15471424, 15469629, 15471417, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[860], &values);

    // Fret Slide
    let values: [i32; 7] = [15471456, 15479239, 15471459, 15479232, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[861], &values);

    // Seashore
    let values: [i32; 7] = [15479271, 15500124, 15479274, 15500116, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[862], &values);

    // Helicopter
    let values: [i32; 7] = [15500156, 15513066, 15500159, 15513058, 44100, 60, 0];
    sample_util::check(&sf.sample_headers[863], &values);
}
