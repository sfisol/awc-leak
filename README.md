# awc-leak

This example fetches `google.com` main site once a second and reports its RSS memory usage upon change printing current iteration number and RSS (in pages, usually 4KB each).

Example output:

```
1. 1346
13. 1864
62. 1886
135. 1918
211. 1950
362. 2014
438. 2046
586. 2111
662. 2143
738. 2175
814. 2207
968. 2272
1117. 2336
1193. 2369
1347. 2435
1501. 2500
1577. 2533
1731. 2599
1882. 2598
1883. 2276
1885. 2319
1961. 2351
2088. 2406
2164. 2438
2240. 2470
2316. 2503
2470. 2568
2546. 2602
2702. 2667
2854. 2733
2930. 2765
3081. 2830
3084. 2832
3238. 2897
3314. 2930
3468. 2995
3619. 3060
3622. 3062
3698. 3094
3852. 3160
4006. 3225
4082. 3257
4180. 3300
4334. 3365
4488. 3431
4564. 3463
4715. 3528
4718. 3530
4872. 3596
4948. 3628
5102. 3693
5253. 3758
5256. 3760
5332. 3792
5486. 3858
5640. 3924
5716. 3957
5870. 4022
6024. 4088
6100. 4120
6251. 4185
6254. 4187
6408. 4253
6484. 4285
6638. 4350
6789. 4415
6792. 4417
6868. 4449
6874. 3961
7022. 4027
7176. 4093
7252. 4126
7310. 4125
7406. 4190
7489. 4179
7560. 4244
7636. 4277
7790. 4343
7944. 4409
8020. 4441
8174. 4506
8195. 4532
8287. 4556
8346. 4544
8363. 4576
8439. 4608
8515. 4641
8591. 4673
8667. 4705
8743. 4739
8895. 4803
8971. 4836
9047. 4868
9123. 4900
9274. 4965
9277. 4967
9431. 5032
9507. 5065
9661. 5130
9812. 5195
9815. 5197
9891. 5229
10045. 5294
10199. 5360
10275. 5394
10418. 5388
10429. 5453
10572. 5416
10583. 5482
10659. 5514
10810. 5579
10813. 5581
10967. 5646
11043. 5679
11197. 5744
11348. 5809
11351. 5811
11427. 5843
11581. 5909
11735. 5974
11811. 6008
11965. 6073
12119. 6139
12195. 6171
12346. 6236
12349. 6238
12505. 6303
12579. 6336
12733. 6401
12884. 6466
12887. 6468
12963. 6500
13117. 6566
13271. 6631
13347. 6665
13501. 6730
13655. 6795
13731. 6828
13882. 6893
13885. 6895
14039. 6960
14115. 6993
14269. 7058
14420. 7123
14423. 7125
14499. 7157
14653. 7223
14807. 7288
14958. 7353
15037. 7387
15191. 7453
15267. 7485
15418. 7550
15421. 7552
15575. 7617
15651. 7650
15805. 7715
15956. 7780
15959. 7782
16035. 7814
16189. 7880
16343. 7945
16385. 7966
16387. 7976
16419. 7993
16573. 8058
16727. 8124
16803. 8156
16954. 8221
16957. 8223
17111. 8288
17187. 8321
17343. 8386
17495. 8453
17571. 8485
17725. 8551
17879. 8616
17955. 8650
18109. 8715
18263. 8781
18339. 8813
18490. 8878
18493. 8880
18647. 8945
18723. 8978
18877. 9043
19028. 9108
19031. 9110
19107. 9142
19261. 9208
19415. 9273
19491. 9307
19645. 9372
19799. 9438
19875. 9470
20026. 9535
20029. 9537
20183. 9602
20259. 9635
20413. 9700
20564. 9765
20567. 9767
20643. 9799
20797. 9865
20953. 9930
21027. 9964
21181. 10029
21335. 10095
21411. 10127
21562. 10192
21565. 10194
21719. 10259
21795. 10292
21949. 10357
22100. 10422
22103. 10424
22259. 10489
22333. 10522
22487. 10587
22563. 10621
22717. 10686
22871. 10752
22947. 10784
23098. 10849
23101. 10851
23255. 10916
23331. 10949
23485. 11014
23636. 11079
23639. 11081
23715. 11113
23869. 11179
24023. 11244
24099. 11278
24253. 11343
24407. 11409
24483. 11441
24634. 11506
24791. 11573
24867. 11606
25021. 11671
25172. 11736
25175. 11738
25251. 11770
25405. 11836
25559. 11901
25635. 11935
25791. 12000
25943. 12066
26019. 12098
26170. 12163
26173. 12165
26327. 12230
26403. 12263
26557. 12328
26708. 12393
26711. 12395
26787. 12427
26941. 12493
27095. 12558
27171. 12592
27325. 12657
27479. 12723
27555. 12755
27706. 12820
27709. 12822
27863. 12887
27939. 12920
28093. 12985
28244. 13050
28323. 13084
28477. 13150
28631. 13215
28707. 13249
28861. 13314
29015. 13380
29091. 13412
29242. 13477
29245. 13479
29399. 13544
29553. 13609
29629. 13642
29780. 13707
29783. 13709
29859. 13741
30013. 13807
30167. 13872
30243. 13906
30397. 13971
30551. 14037
30627. 14069
30778. 14134
30781. 14136
30935. 14201
31011. 14234
31165. 14299
31316. 14364
31319. 14366
31395. 14398
31549. 14464
31703. 14529
31779. 14563
31933. 14628
32087. 14694
32163. 14726
32314. 14791
32317. 14793
32471. 14858
32547. 14891
32701. 14956
32771. 14987
32855. 15023
32931. 15055
33085. 15120
33239. 15186
33315. 15220
33469. 15285
33623. 15351
33699. 15383
33850. 15448
33853. 15450
34007. 15515
34083. 15548
34237. 15613
34388. 15678
34391. 15680
34467. 15712
34621. 15778
34691. 15777
34775. 15842
34851. 15876
35005. 15941
35159. 16007
35235. 16039
35386. 16104
35389. 16106
35545. 16171
35619. 16204
35773. 16269
35924. 16334
35927. 16336
36003. 16368
36157. 16434
36311. 16499
36387. 16533
36541. 16598
36695. 16664
36851. 16729
36925. 16763
37079. 16828
37155. 16861
37309. 16926
37460. 16991
37463. 16993
37539. 17025
37693. 17091
37847. 17156
37998. 17221
38077. 17255
38231. 17321
38307. 17353
38458. 17418
38461. 17420
38615. 17485
38638. 17484
38691. 17517
38845. 17582
38996. 17647
38999. 17649
39075. 17681
39229. 17747
39383. 17812
39459. 17846
39613. 17911
39767. 17977
39843. 18009
39994. 18074
39997. 18076
40151. 18141
40227. 18174
40383. 18239
40535. 18306
40611. 18338
40765. 18404
40919. 18469
40995. 18503
41149. 18568
41303. 18634
41379. 18666
41530. 18731
41533. 18733
41687. 18798
41763. 18831
41917. 18896
42068. 18961
42071. 18963
42147. 18995
42301. 19061
42455. 19126
42531. 19160
42685. 19225
42839. 19290
42915. 19323
43066. 19388
43069. 19390
43223. 19455
43299. 19488
43453. 19553
43604. 19618
43607. 19620
43683. 19652
43837. 19718
43993. 19783
44067. 19817
44221. 19882
44375. 19948
44451. 19980
44602. 20045
44605. 20047
44759. 20112
44835. 20145
44989. 20210
45140. 20275
45143. 20277
45299. 20342
45373. 20375
45527. 20440
45603. 20474
45757. 20539
45911. 20605
45987. 20637
46138. 20702
46141. 20704
46295. 20769
46371. 20802
46525. 20867
46676. 20932
46679. 20934
46755. 20966
46909. 21032
47063. 21097
47139. 21131
47293. 21196
47447. 21262
47523. 21294
47674. 21359
47831. 21426
47907. 21459
48061. 21524
48212. 21589
48215. 21591
48291. 21623
48445. 21689
48599. 21754
48675. 21788
48829. 21853
48983. 21919
49059. 21951
49210. 22016
49213. 22018
49367. 22083
49443. 22116
49597. 22181
49748. 22246
49751. 22248
49827. 22280
49981. 22346
50135. 22411
50211. 22445
50365. 22510
50519. 22576
50595. 22608
50746. 22673
50749. 22675
50903. 22740
50979. 22773
51133. 22838
51284. 22903
51363. 22937
51517. 23003
51671. 23068
51747. 23102
51901. 23167
52055. 23233
52131. 23265
52282. 23330
52285. 23332
52439. 23397
```