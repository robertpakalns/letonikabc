# LetonikaBC
Letonika Better Client 

Tīmekļa vietne, kurā lietotājs lejupielādē teksta failu no [letonika.lv](https://letonika.lv) vietnes un izmanto kā alternatīvu lasītāvu.

## Sistēmas vienkāršots modelis  

###  Modeļa objekti  

* Teksta fails no `letonika.lv`  
Sākotnējais saturs, ko lietotājs importē no savas ierīces un izmanto lasīšanai
* Apstrādāts dokuments  
Sistēmas izveidots, lietošanai pielāgots dokuments, ar kuru lietotājs strādā vietnē
* Meklēšanas filtrs  
Modulis, kas ļauj lietotājam atrast tekstu dokumentā
* Iebūvētā datubāze  
Pārlūkprogrammas lokālā krātuve, kur tiek saglabāti dokumenti un informācija par tiem
* Dokumentu apstrādes modulis  
Iekšējais sistēmas mehānisms, kas pārveido sākotnējo tekstu lietotājam draudzīgā formā

### Modeļa objektu saites
1. LIETOTĀJS lejupielādē TEKSTA FAILU
2. TEKSTA APSTRĀDES MODULIS apstrādā TEKSTA FAILU
3. TEKSTA APSTRĀDES MODULIS izveido APSTRĀDĀTO DOKUMENTU
4. APSTRĀDĀTS DOKUMENTS tiek saglabāts IEBŪVĒTĀ DATUBĀZĒ; IEBŪVĒTĀ DATUBĀZE atgriež saglabāto APSTRĀDĀTO DOKUMENTU
5. LASĪTAVA pieprasa dokumentu IEBŪVĒTĀ DATUBĀZĒ
6. LIETOTĀJS izmanto LASĪTAVU
7. LASĪTAVA atgriež tekstu TEKSTA APSTRĀDES MODULIM
8. TEKSTA APSTRĀDES MODULIS saglabā apstrādāto tekstu IEBŪVĒTĀ DATUBĀZE
9. LIETOTĀJS lasa APSTRĀDĀTO DOKUMENTU
10. LIETOTĀJS izmanto TEKSTA APSTRĀDES MODUĻA filtrēšanas funkcijas

## Datu vārdnīca

### Oriģināls teksta fails
| Lauks  | Tips   | Apraksts                 |
| ------ | ------ | ------------------------ |
| ID     | number | Dokumenta identifikators |
| Izmērs | number | Rakstzīmju skaits        |
| Saturs | string | Pilns teksta saturs      |

### Apstrādāts dokuments
| Lauks     | Tips   | Apraksts           |
| --------- | ------ | ------------------ |
| ID        | number | Dokumenta ID       |
| Izmērs    | number | Teksta garums      |
| Saturs    | string | Saglabātais teksts |
| Nosaukums | string | Var būt tukšs      |
| Autors    | string | Var būt tukšs      |

### Metadati
| Lauks               | Tips   | Apraksts                    |
| ------------------- | ------ | --------------------------- |
| ID                  | number | Primārā atslēga             |
| Sākotnējais izmērs  | number | Oriģinālais teksta izmērs   |
| Apstrādātais izmērs | number | Teksta izmērs pēc apstrādes |
| Nosaukums           | string | Var būt tukšs               |
| Autors              | string | Var būt tukšs               |
| Izveides laiks      | Date   | ISO 8601 standarts          |
| Rediģēšanas laiks   | Date   | ISO 8601 standarts          |

## Funkcionālās prasības
1. Sistēma ļauj lietotājam augšupielādēt teksta failu no savas ierīces.
2. Sistēma pārbauda, vai fails ir derīgs turpmākai apstrādei.
3. Sistēma sagatavo failu lietošanai un parāda to lietotāja saskarnē.
4. Sistēma automātiski iegūst dokumenta pamatinformāciju (ID, nosaukumu, autoru, izmēru utt.).
5. Sistēma saglabā dokumentu iebūvētajā datubāzē turpmākai lietošanai.
6. Sistēma parāda lietotājam saglabāto dokumentu sarakstu.
7. Sistēma ļauj lietotājam atvērt izvēlēto dokumentu lasīšanas režīmā.
8. Sistēma nodrošina teksta meklēšanu dokumenta saturā.
9. Sistēma ļauj lietotājam ierobežot meklēšanu uz izvēlētām teksta daļām.
10. Sistēma ļauj lietotājam labot dokumenta nosaukumu un autoru.
11. Sistēma ļauj importēt un eksportēt dokumentus `.md` formātā.

## Nefunkcionālās prasības

### Prasības procesam
* Strukturēta moduļu arhitektūra
* Stingra tipizācija lietotāja saskarnes izstrādē
* Automatizēta testēšana teksta apstrādes un meklēšanas funkcijām
* Datu atjaunināšana bez lietotāja saglabāto datu bojāšanas
* Automātiska sistēmas būvēšana un izvietošana

### Prasības produktam
* Sistēmas Lietotāja saskarne reaģē ne vēlāk kā 0.5 sekunžu laikā
* Sistēma apstrādā tekstu ne lēnāk kā 3 sekunžu laikā
* Sistēma darbojas bez papildu programmu instalēšanas
* Sistēma spēj darboties bez interneta pieslēguma

### Ārējās prasības
Operētājsistēmas, kuras atbalstīs sistēmu:
* Windows 11
* Linux (Lubuntu 24.04)
* Android 15

Pārlūkprogrammas, kuras atbalstīs sistēmu:
* Google Chrome
* Mozilla Firefox
* Microsoft Edge

### Nefunkcionālās prasības 5 funkcijām
* Teksta faila importēšana
  * Maksimālais faila ielādes laiks: 3 sekundes
  * Atbalstītais faila izmērs: līdz 15 MB

* Dokumenta saglabāšana
  * Saglabāšanas laiks: līdz 1 sekundei

* Dokumenta atvēršana
  * Atvēršanas laiks: līdz 0.5 sekundēm
  * Navigācijai jāparādās automātiski

* Teksta meklēšana
  * Rezultātu parādīšanas laiks: līdz 1 sekundei
  * Jāatbalsta vismaz 100,000 simbolu apjoms

* Dokumentu imports/eksports
  * Faila apstrādes laiks: līdz 3 sekundēm
