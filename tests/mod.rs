use typoscale::*;
// print(json.dumps([f"{i} {tenths[int(j*10)]}".strip() for i, j in [(math.floor(typoscale(i)), math.fmod(typoscale(i),1.0)) for i in range(0,64)]], ensure_ascii=False))
const RESULTS: &[&str] = &["1", "1 1⁄10", "1 3⁄10", "1 ½", "1 7⁄10", "2", "2 ⅕", "2 ⅗", "3", "3 ⅖", "4", "4 ½", "5 ⅕", "6", "6 9⁄10", "8", "9 1⁄10", "10 ½", "12 1⁄10", "13 9⁄10", "16", "18 3⁄10", "21 1⁄10", "24 ⅕", "27 ⅘", "32", "36 7⁄10", "42 ⅕", "48 ½", "55 7⁄10", "64", "73 ½", "84 ⅖", "97", "111 ⅖", "128", "147", "168 ⅘", "194", "222 ⅘", "256", "294", "337 7⁄10", "388", "445 7⁄10", "512", "588 1⁄10", "675 ½", "776", "891 ⅖", "1024", "1176 ⅕", "1351 1⁄10", "1552", "1782 ⅘", "2048", "2352 ½", "2702 3⁄10", "3104 1⁄10", "3565 7⁄10", "4096", "4705", "5404 7⁄10", "6208 3⁄10"];
// print(json.dumps([typoscale(i) for i in range(64)]))
const RESULTS2: &[f64] = &[1.0, 1.148698354997035, 1.3195079107728942, 1.515716566510398, 1.7411011265922482, 2.0, 2.2973967099940698, 2.6390158215457884, 3.0314331330207964, 3.4822022531844965, 4.0, 4.59479341998814, 5.278031643091577, 6.062866266041593, 6.964404506368992, 8.0, 9.18958683997628, 10.556063286183154, 12.125732532083186, 13.928809012737984, 16.0, 18.37917367995256, 21.112126572366314, 24.251465064166364, 27.85761802547597, 32.0, 36.75834735990512, 42.22425314473263, 48.50293012833273, 55.71523605095194, 64.0, 73.51669471981025, 84.44850628946526, 97.00586025666546, 111.43047210190387, 128.0, 147.0333894396205, 168.89701257893051, 194.0117205133309, 222.86094420380775, 256.0, 294.0667788792408, 337.79402515786103, 388.0234410266618, 445.7218884076158, 512.0, 588.1335577584816, 675.5880503157221, 776.0468820533237, 891.4437768152316, 1024.0, 1176.2671155169633, 1351.1761006314441, 1552.0937641066473, 1782.8875536304631, 2048.0, 2352.5342310339265, 2702.3522012628882, 3104.1875282132946, 3565.7751072609262, 4096.0, 4705.068462067853, 5404.7044025257765, 6208.375056426589];

#[test]
fn fraction_str() {
    for i in 0..64 {
        assert_eq!(i.fraction_str(), RESULTS[i]);
    }
}

#[test]
fn typoscale() {
    for i in 0..64 {
        assert_eq!(i.typoscale(), RESULTS2[i]);
    }
}

#[test]
fn iter() {
    assert_eq!(iter::TypoScaleIterator::default().take(64).map(|f|f as usize).collect::<Vec<_>>().as_slice(), RESULTS2.into_iter().map(|f|*f as usize).collect::<Vec<_>>().as_slice());
}

#[test]
fn iter_lcg() {
    let i = iter::TypoScaleLcgIterator::default();
    let n = i.take(24).collect::<Vec<_>>();
    assert_eq!(n, [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 14, 16, 18, 21, 24, 30, 36, 48, 60, 72, 84, 96, 128]);
}
