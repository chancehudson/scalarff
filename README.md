# scalarff [![CircleCI](https://dl.circleci.com/status-badge/img/gh/chancehudson/scalarff/tree/main.svg?style=shield)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/scalarff/tree/main)

A minimal, opinionated, library for working with scalar finite fields.

## Usage

This library exports a [`FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html) trait and concrete implementations for the following curves:

- `FoiFieldElement` - `2^64 - 2^32 + 1` field element [powered by](https://docs.rs/twenty-first/latest/twenty_first/math/b_field_element/struct.BFieldElement.html)
- `Curve25519FieldElement` - `curve25519` field element [powered by](https://docs.rs/curve25519-dalek/latest/curve25519_dalek/scalar/index.html)
- `Bn128FieldElement` - `alt_bn128` field element [powered by](https://docs.rs/ark-bn254/0.4.0/ark_bn254/)

See the [1000 residues](https://github.com/chancehudson/scalarff/blob/main/examples/1000_residues.rs) example to get started.

```
finding the next 10 residues in field alt_bn128: starting at 360
    361_alt_bn128 = 19 * 21888242871839275222246405745257275088548364400416034343698204186575808495598
    362_alt_bn128 = 7410792142369690883099566644927187788573007891049740207083319522829291451145 * 14477450729469584339146839100330087299975356509366294136614884663746517044472
    363_alt_bn128 = 48487130673258682271209282342166764461106410045129796317117 * 21888242871839275173759275071998592817339082058249269882591794141446012178500
    373_alt_bn128 = 6741263649621776060598491874640695270456144096494402856975381499935570164766 * 15146979222217499161647913870616579818092220303921631486722822686640238330851
    374_alt_bn128 = 7746079973149552665931043950370262039712588441029222121622872378646431859442 * 14142162898689722556315361794887013048835775959386812222075331807929376636175
    377_alt_bn128 = 6720256710434864407550813627806045044281405569601070346199302561280361353611 * 15167986161404410814695592117451230044266958830814963997498901625295447142006
    380_alt_bn128 = 373948076583382765639247755170945072830789704575629160411641175459419747546 * 21514294795255892456607157990086330015717574695840405183286563011116388748071
    382_alt_bn128 = 6227098698075769680927133531450739058997850923239544522494417437193297873685 * 15661144173763505541319272213806536029550513477176489821203786749382510621932
    383_alt_bn128 = 1989959376781032188476446759931244730407324713239690028026002251695217985587 * 19898283495058243033769958985326030358141039687176344315672201934880590510030
    384_alt_bn128 = 2246183851384062820462584043901179168868401833741124742999088669201012712522 * 19642059020455212401783821701356095919679962566674909600699115517374795783095
^^^^^^^^^^ function excecuted in 23 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
finding the next 10 residues in field curve25519: starting at 360
    361_curve25519 = 19 * 7237005577332262213973186563042994240857116359379907606001950938285454250970
    362_curve25519 = 2840544616763072301696963885412918603553685993119059342516150524306355217261 * 4396460960569189912276222677630075637303430366260848263485800413979099033728
    363_curve25519 = 841279760723326825641141251718813592146471985329058982431655168532313770692 * 6395725816608935388332045311324180648710644374050848623570295769753140480297
    364_curve25519 = 3067445422133940858093415918505373430248281200094250477803124667985072644135 * 4169560155198321355879770644537620810608835159285657128198826270300381606854
    365_curve25519 = 2229020166559983688306808155563766780306655457832930424277443510759676986617 * 5007985410772278525666378407479227460550460901546977181724507427525777264372
    369_curve25519 = 977769322213384990377615820422500475977525553269069653071645090181089469610 * 6259236255118877223595570742620493764879590806110837952930305848104364781379
    371_curve25519 = 1267588560776674033127942769472030462887504776849313608245295767874810858995 * 5969417016555588180845243793570963777969611582530593997756655170410643391994
    373_curve25519 = 119454269714651670553073425880206504907017102091599218183277904282820570066 * 7117551307617610543420113137162787735950099257288308387818673034002633680923
    375_curve25519 = 1611903041274636972169170005108898995248235655877685844541292224499120185464 * 5625102536057625241804016557934095245608880703502221761460658713786334065525
    377_curve25519 = 631896535449647541813070049528266238057309155245325651374203027632474016658 * 6605109041882614672160116513514728002799807204134581954627747910652980234331
^^^^^^^^^^ function excecuted in 3 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
finding the next 10 residues in field 0xfoi: starting at 360
    360_0xfoi = 04886810760654287587 * 13559933308760296734
    361_0xfoi = 19 * -19
    363_0xfoi = 00003096224742375424 * 18443647844672208897
    364_0xfoi = 00640366319723949669 * 17806377749690634652
    368_0xfoi = 08139125605395827597 * 10307618464018756724
    369_0xfoi = 03284662639461963411 * 15162081429952620910
    371_0xfoi = 02993791755975720565 * 15452952313438863756
    373_0xfoi = 08308875621651992349 * 10137868447762591972
    375_0xfoi = 08637146607354536426 * 09809597462060047895
    384_0xfoi = 01152912708379604992 * 17293831361034979329
^^^^^^^^^^ function excecuted in 3 ms ^^^^^^^^^^
||||||||||||||||||||||||||||||||||||||||
10 residues in alt_bn128 excecuted in 23 ms
10 residues in curve25519 excecuted in 3 ms
10 residues in 0xfoi excecuted in 3 ms
```
