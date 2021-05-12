import matplotlib.pyplot as plt

if __name__ == '__main__':
    y1 = [1990.9995982322378, 1591.8226441772038, 920.237056077846, 561.852971160593, 384.2655006173302, 272.44000667746394, 196.0607769068905, 142.47152682885755, 104.26247815987895, 76.7343138849766, 56.74325980538619, 42.13426578450131, 31.401706530724777, 23.480938568687655, 17.61159609843378, 13.24649586811297, 9.989301642223202, 7.551361034304002, 5.7214242643760915, 4.344218254697288, 3.3051642098434284, 2.5194073192960906, 1.923896119577659, 1.4716348171719196, 1.1274932242330764, 0.8651394313469113, 0.6647857183322642, 0.5115261744203659, 0.3941066167711461, 0.30401154639959593, 0.23478443891734443, 0.18152033643226145, 0.14048606966816404, 0.10883529957216637, 0.08439419872182419, 0.06549989884076551, 0.050878452212928055, 0.03955245296425347, 0.030770974898069264, 0.023956333872612173, 0.018663564302982633, 0.014549519684845275, 0.011349274143565957, 0.008858071206256785, 0.00691749607450165, 0.005404867846355382, 0.004225090525500436, 0.003304385900260026, 0.0025854665895903572, 0.002023814458819495, 0.0015848084768341764, 0.0012415049320607108, 0.0009729205665372775, 0.0007627035676131266, 0.0005981033071104369, 0.0004691716716994643, 0.0003681423260576411, 0.00028894957982236735, 0.00022685307144945654, 0.0001781468342582082, 0.00013993198109402138, 0.00010994021731947079, 0.00008639567859816677, 0.0000679076557195124, 0.00005338665427834233, 0.00004197882714687684, 0.00003301469944014257, 0.00002596925360831559, 0.00002043077785818248, 0.000016076062522268897, 0.000012651481377606855, 0.000009957881379724087, 0.00000783882148761017, 0.000006171530015419702, 0.000004859515489646915, 0.000003826902902709373, 0.0000030140705688241898, 0.000002374119677905906, 0.0000018702653059865515, 0.000001473489905645664, 0.0000011609909696674858, 0.0000009149152881693379, 0.0000007210083924347188, 0.0000005682953581886974, 0.0000004478865628654294, 0.00000035309512103065543, 0.00000027835012331167697, 0.00000021943794731305033, 0.00000017303200713914002, 0.00000013646400093270117, 0.0000001075840966036079, 0.00000008483475272269558, 0.00000006691983447937311, 0.000000052775949666017397, 0.00000004163272859791789, 0.00000003283282165589263, 0.00000002592565115366874, 0.000000020441541959126397, 0.000000016098597593883213, 0.000000012724926223839361, 0.000000010028664482186755, 0.000000007927058676138188, 0.000000006252181994348405, 0.000000004918935675446434, 0.000000003882386606646193, 0.000000003092526623671432, 0.0000000024258958453149404, 0.0000000019064185508010922, 0.0000000015160750710396798, 0.0000000011892895279697058, 0.0000000009436340042690006, 0.0000000007395387635611428, 0.0000000005935261726541796, 0.0000000004608550485674101, 0.0000000003641921764163669, 0.0000000002929961828712635, 0.00000000021819240858533817, 0.0000000001762397450200126, 0.00000000013576995083752763, 0.00000000010645567760647623, 0.00000000010589007448658094, 0.00000000007234307597414613, 0.00000000006170383648473887, 0.000000000049945048097299605, 0.000000000027736701824210286, 0.000000000026964069865798024, 0.000000000015911688633352128, 0.000000000017043699784835553, 0.000000000014865830788579615, 0.0000000000094876329015392, 0.0000000000020803914146938496, 0.0000000000012909950886097477, 0.0000000000029628244302415396, 0.0000000000016137646774438963, 0.0000000000010406675521323905, 0.00000000000012193024367945782, 0.00000000000003122502256758253, 0.00000000000002808864252301646, 0.000000000000012184697695261093, 0.000000000000003552713678800501, 0.0000000000000009992007221626409, 0.000000000000000943689570931383, 0.0000000000000006106226635438361, 0.00000000000000127675647831893]
    y2 = [1990.9991227647654, 1590.63886354491, 447.322829964295, 101.56265358508973, 49.257662616514196, 33.241031407889, 23.38322869328756, 16.63910866378581, 11.886175559947592, 8.50361776821262, 6.087417653883172, 4.358901884277165, 3.12155690015214, 2.2355669689013684, 1.6010838862273296, 1.1466872543530537, 0.8212547975368618, 0.5881820661012338, 0.4212559600968224, 0.3017036264444146, 0.21608025805122938, 0.15475678163730217, 0.11083688388219495, 0.0793814327965503, 0.05685302318111801, 0.04071816478177123, 0.029162370854122432, 0.020886105204842603, 0.014958639441801508, 0.010713385399317488, 0.007672932178676262, 0.0054953580475749775, 0.003935778300073772, 0.002818806490633513, 0.0020188306460965644, 0.001445887611878277, 0.0010355454096260541, 0.0007416581998593019, 0.0005311759913958769, 0.0003804285080977743, 0.00027246309357892295, 0.00019513821257319264, 0.00013975809906136005, 0.00010009482684411597, 0.00007168796072545836, 0.000051342957246625076, 0.00003677184141864798, 0.00002633601593632351, 0.000018861868667480186, 0.000013508889747493313, 0.000009675071958981096, 0.000006929285660095541, 0.000004962743499453692, 0.0000035543418887007405, 0.000002545626261035716, 0.0000018231604690943648, 0.0000013057526360993332, 0.0000009351766096787895, 0.0000006697939878008707, 0.0000004796718096888863, 0.000000343575174532873, 0.00000024605268528210544, 0.0000001762119040404908, 0.00000012622506304071912, 0.00000009039470616478695, 0.00000006473887598223982, 0.0000000463655762672488, 0.00000003321846589465771, 0.000000023781139307788024, 0.000000017023819937067586, 0.000000012204776356217195, 0.000000008749151125808652, 0.000000006252436124398741, 0.0000000044798397491607744, 0.0000000032084191969872222, 0.0000000022860076887010194, 0.000000001659042403456823, 0.0000000011758549411933217, 0.0000000008477449020549699, 0.0000000006118109074915168, 0.00000000042638712005604873, 0.00000000029667954204448677, 0.00000000023932411608029724, 0.00000000015319862045615196, 0.00000000011268758148830216, 0.00000000008173914323172937, 0.00000000006532160923278241, 0.00000000004236608286412036, 0.000000000028650692929232946, 0.000000000018182455541193576, 0.00000000001487354683860076, 0.000000000007844225269337812, 0.000000000007788020228716164, 0.000000000004978073508965508, 0.0000000000027485513864888844, 0.0000000000057791826879594055, 0.000000000004603650793910674, 0.0000000000033138491950523985, 0.0000000000008118505867571457, 0.0000000000003634592626866606, 0.00000000000005440092820663267, 0.00000000000003430589146091734, 0.0000000000000014988010832439613, 0.000000000000001887379141862766, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
    y3 = [0.8887613302610946, 0.34394880313333415, 0.22184590497470613, 0.09668437582893788, 0.05636116493680667, 0.042112137569663234, 0.03444055175758526, 0.029084888103055553, 0.024982538328120443, 0.021725028566660788, 0.01906436284778219, 0.016852803499295372, 0.014987673898160648, 0.01339806345287298, 0.012031252591714642, 0.01084761212448483, 0.00981627979088325, 0.008912900006838202, 0.00811788749508112, 0.0074153173761068595, 0.006792090527632836, 0.006237338588802009, 0.005741970708020263, 0.0052983309059185316, 0.004899931390181252, 0.004541243906910834, 0.004217533991429027, 0.003924728309144393, 0.003659307444830754, 0.003418218648716963, 0.0031988043125108314, 0.002998742981566681, 0.002816000421836103, 0.002648788813255392, 0.002495532550405131, 0.0023548394476470384, 0.0022254763884191346, 0.002106348647634525, 0.0019964822642723444, 0.001895008958262926, 0.001801153178765314, 0.0017142209451850427, 0.0016335902019837222, 0.0015587024565123166, 0.0014890555081948777, 0.0014241971092167642, 0.0013637194229281842, 0.0013072541675896014, 0.0012544683506757449, 0.0012050605136081536, 0.0011587574188600448, 0.001115311121507762, 0.0010744963757330112, 0.0010361083338729167, 0.000999960501571892, 0.0009658829176457347, 0.0009337205314914989, 0.000903331754555759, 0.0008745871654206518, 0.0008473683507244343, 0.0008215668663894259, 0.0007970833055892076, 0.0007738264615398017, 0.000751712574676164, 0.0007306646549959288, 0.0007106118714814366, 0.0006914890014231909, 0.0006732359333165814, 0.000655797217718578, 0.0006391216610911844, 0.0006231619581985293, 0.000607874359135329, 0.0005932183674732603, 0.0005791564664091571, 0.0005656538701151138, 0.0005526782978135099, 0.000540199768329328, 0.0005281904131406966, 0.0005166243061245727, 0.000505477308404325, 0.000494726926848346, 0.0004843521849337652, 0.00047433350480568804, 0.0004646525994869301, 0.0004552923742964022, 0.0004462368366217282, 0.00043747101328510586, 0.0004289808748068776, 0.0004207532659436666, 0.00041277584193896727, 0.0004050370099705259, 0.00039752587533769125, 0.00039023219197119536, 0.00038314631688310825, 0.00037625916821627846, 0.00036956218658331334, 0.0003630472994102146, 0.00035670688803155073, 0.00035053375729906594, 0.0003445211075005595, 0.0003386625083854792, 0.00033295187513404434, 0.00032738344610052457, 0.0003219517621905929, 0.0003166516477383749, 0.0003114781927640828, 0.0003064267364983437, 0.0003014928520779701, 0.0002966723323143833, 0.0002919611764570378, 0.000287355577871213, 0.00028285191255802446, 0.00027844672845840087, 0.00027413673547337753, 0.00026991879615582807, 0.00026578991701296665, 0.00026174724038620795, 0.00025778803685530175, 0.0002539096981344119, 0.0002501097304234099, 0.00024638574818125215, 0.0002427354682902455, 0.0002391567045867391, 0.00023564736272845837, 0.00023220543537798942, 0.0002288289976801474, 0.0002255162030109575, 0.00022226527898357607, 0.00021907452368995908, 0.00021594230216420536, 0.00021286704305332078, 0.00020984723548247752, 0.00020688142609727168, 0.00020396821627739322, 0.0002011062595094435, 0.00019829425890417486, 0.00019553096485408098, 0.0001928151728221573, 0.000190145721248398, 0.00018752148957499934, 0.00018494139637448257, 0.0001824043975807756, 0.00017990948481454483, 0.00017745568379586466, 0.00017504205284159247, 0.00017266768144004636, 0.00017033168890114188, 0.00016803322307397183, 0.00016577145913112886, 0.00016354559841456472, 0.00016135486733909356, 0.0001591985163509449, 0.00015707581893780452, 0.00015498607068799023, 0.00015292858839577105, 0.00015090270920930688, 0.00014890778982032943, 0.00014694320569306156, 0.00014500835032930042, 0.00014310263456762906, 0.00014122548591702223, 0.00013937634792064247, 0.00013755467954837947, 0.00013575995461921893, 0.00013399166124832224, 0.00013224930131981572, 0.00013053238998352726, 0.00012884045517348267, 0.0001271730371492291, 0.00012552968805584866, 0.00012390997150469089, 0.00012231346217078635, 0.00012073974540881782, 0.00011918841688623752, 0.00011765908222995987, 0.0001161513566899346, 0.00011466486481576696, 0.00011319924014793738, 0.0001117541249205748, 0.00011032916977741972, 0.00010892403349953561, 0.00010753838274283734, 0.0001061718917882058, 0.00010482424229967355, 0.00010349512309375765, 0.00010218422991730451, 0.00010089126523390046, 0.00009961593801886141, 0.00009835796356243945, 0.00009711706328032506, 0.00009589296453085302, 0.00009468540044111772, 0.00009349410973695467, 0.00009231883658116297, 0.00009115933041755002, 0.00009001534582009202, 0.00008888664234806064, 0.00008777298440668938, 0.00008667414111204161, 0.0000855898861618684]

    # Draw Plot
    fig = plt.figure(dpi=200)
    plt.plot(range(80), y1[:80], '-', label='PageRank with d. f.', color='xkcd:aquamarine')
    plt.plot(range(80), y2[:80], '-', label='Weighted PageRank', color='xkcd:red')
    plt.plot(range(80), y3[:80], '-', label='Basic PageRank', color='xkcd:green')
    plt.yscale('log')
    plt.xlabel('Number of iterations')
    plt.ylabel('Distance between rank vectors')
    plt.legend(loc=3, fontsize=12)

    # Decoration
    plt.xlim(xmax=80)
    xtick_location = [10 * i for i in range(9)]
    plt.xticks(ticks=xtick_location, labels=xtick_location, rotation=0, fontsize=12, horizontalalignment='center', alpha=.7)
    plt.yticks(fontsize=12, alpha=.8)
    plt.title("PageRank convergence", fontsize=22)
    plt.grid(axis='both', alpha=.3)

    # Remove borders
    plt.gca().spines["top"].set_alpha(0.0)
    plt.gca().spines["bottom"].set_alpha(0.3)
    plt.gca().spines["right"].set_alpha(0.0)
    plt.gca().spines["left"].set_alpha(0.3)


    fig.savefig('./pagerank-convergence.png')
    fig.show()