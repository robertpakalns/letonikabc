# LetonikaBC
Letonika Better Client 

Tīmekļa vietne, kurā lietotājs lejupielādē teksta failu no [letonika.lv](https://letonika.lv) vietnes un izmanto kā alternatīvu lasītāvu

## Sistēmas vienkāršots modelis  

###  Modeļa objekti  

![Modeļa objekti un to saites](https://raw.githubusercontent.com/robertpakalns/letonikabc/main/assets/readme-saites.svg)

### Modeļa objektu saites
1. `TEKSTA MODULIS` apstrādā `TEKSTA FAILU` un izveido `APSTRĀDĀTO DOKUMENTU`
2. `LASĪTAVA` pieprasa dokumentu `IEBŪVĒTĀ DATUBĀZĒ`
3. `IEBŪVĒTĀ DATUBĀZE` atgriež `APSTRĀDĀTO DOKUMENTU`, izmantojot `METADATUS`
4. `LIETOTĀJS` izmanto `APSTRĀDĀTO DOKUMENTU` lasīšanai
5. `TEKSTA MODULIS` saņem `METADATUS` no `TEKSTA FAILA`
6. `LIETOTĀJS` izmanto `LASĪTAVU` lasīšanai
7. `LIETOTĀJS` var mainīt `METADATUS`
8. `LIETOTĀJS` var lejupielādēt `TEKSTA FAILU`
9. `LASĪTAVA` nodod `TEKSTA FAILU` tālākai apstrādei `TEKSTA MODULIM`
10. `APSTRĀDĀTS DOKUMENTS` tiek saglabāts `IEBŪVĒTĀ DATUBĀZĒ`
11. `TEKSTA MODULIS` izmanto `APSTRĀDĀTO DOKUMENTU` teksta meklēšanai
12. `METADATI` tiek saglabāti `IEBŪVĒTĀ DATUBĀZĒ`

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
| Pēdējā pozīcija     | number | Lasīšanas pēdējā pozīcija   |

## Funkcionālās prasības
1. Sistēma ļauj lietotājam augšupielādēt teksta failu no savas ierīces.
2. Sistēma pārbauda, vai fails ir derīgs turpmākai apstrādei.
3. Sistēma automātiski iegūst un saglabā dokumenta pamatinformāciju (ID, oriģinālā un jaunā faila izmēru, lietotāja pēdējo pozīciju dokumentā utt.).
4. Sistēma saglabā dokumentu iebūvētajā datubāzē turpmākai lietošanai.
5. Sistēma parāda lietotājam saglabāto dokumentu sarakstu.
6. Sistēma ļauj lietotājam atvērt izvēlēto dokumentu lasīšanas režīmā.
7. Sistēma nodrošina teksta meklēšanu dokumenta saturā.
8. Sistēma ļauj lietotājam ierobežot meklēšanu uz izvēlētām teksta daļām.
9. Sistēma ļauj lietotājam pievienot un labot dokumenta nosaukumu un autoru.
10. Sistēma ļauj importēt un eksportēt dokumentus `.md` formātā.

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
  * Faila ielādes laiks: līdz 3 sekundēm
  * Faila apstrādes laiks: līdz 3 sekundēm
  * Atbalstītais faila izmērs: līdz 15 MB

* Dokumenta saglabāšana
  * Oriģinālais teksta fails netiek izmantots, lai samazinātu atmiņas patēriņu
  * Saglabāšanas laiks: līdz 1 sekundei

* Dokumenta atvēršana
  * Lietotājs var atsākt lasīšanu no pēdējās pozīcijas
  * Atvēršanas laiks: līdz 0.5 sekundēm
  * Navigācijai jāparādās automātiski

* Teksta meklēšana
  * Rezultātu parādīšanas laiks: līdz 1 sekundei
  * Jāatbalsta vismaz 100,000 simbolu apjoms
  * Meklēšanas funkcijai jāatbalsta reģistra jutība (case-sensitive) un reģistra nejutība (case-insensitive), kā arī diakritisko zīmju atšķirību ņemšana vērā

* Dokumentu imports/eksports
  * Sistēma atbalsta tikai `.md` faila formātu
  * Sistēma pievieno galveni ar metadatiem eksportējamā `.md` failā un lasa to importējamā `.md` failā
  * Faila apstrādes laiks: līdz 3 sekundēm
  * Sistēma paziņo lietotājam par kļūdām, ja tās ir radušas importēšanas/eksportēšanas laikā
