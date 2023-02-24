pub type Colour = u8;

pub const GREY: Colour = 0;

// pub const EDGE_COLOURS: [Colour; 5] = [
//     1, // Orange background, cyan inner.
//     9, // Dark Blue background, yellow circular flower.
//     17, // Pink background, cyan mid, pink inner.
//     5, // Green background, thin dark blue circle.
//     13 // Maroon background, orange cross.
// ];

// pub const MID_COLOURS: [Colour; 17] = [
//     2, // Pink background, thin yellow cross.
//     10, // Purple background, fat cyan cross.
//     18, // Yellow background, light blue star.
//     6, // Purple background, purple plus in yellow circle.
//     14, // Green background, thin pink cross.
//     3, // Maroon background, maroon plus in green circle.
//     11, // Green background, fat orange cross.
//     19, // Maroon background, yellow star.
//     7, // Cyan background, pink castle.
//     15, // Yellow background, thin green, hollow square.
//     4, // Cyan background, fat pink cross.
//     12, // Yellow background, dark blue castle.
//     20, // Orange background, purple star.
//     8, // Dark blue background, thin orange cross.
//     16, // Dark blue background, light blue square.
//     21, // Pink background, yellow castle.
//     22 // Dark blue background, dark blue cross in pink circle.
// ];

pub const BUCAS_LETTER: [char; 23] = [
    'a', //  0 Grey edge. EDGE ONLY.
    'b', //  1 Orange background, cyan inner. EDGE ONLY.
    'c', //  9 Dark Blue background, yellow circular flower. EDGE ONLY.
    'd', // 17 Pink background, cyan mid, pink inner. EDGE ONLY.
    'e', //  5 Green background, thin dark blue circle. EDGE ONLY.
    'f', // 13 Maroon background, orange cross. EDGE ONLY.
    'g', //  2 Pink background, thin yellow cross. MID ONLY.
    'h', // 10 Purple background, fat cyan cross. MID ONLY.
    'i', // 18 Yellow background, light blue star. MID ONLY.
    'j', //  6 Purple background, purple plus in yellow circle. MID ONLY.
    'k', // 14 Green background, thin pink cross. MID ONLY.
    'l', //  3 Maroon background, maroon plus in green circle. MID ONLY.
    'm', // 11 Green background, fat orange cross. MID ONLY.
    'n', // 19 Maroon background, yellow star. MID ONLY.
    'o', //  7 Cyan background, pink castle. MID ONLY.
    'p', // 15 Yellow background, thin green, hollow square. MID ONLY.
    'q', //  4 Cyan background, fat pink cross. MID ONLY.
    'r', // 12 Yellow background, dark blue castle. MID ONLY.
    's', // 20 Orange background, purple star. MID ONLY.
    't', //  8 Dark blue background, thin orange cross. MID ONLY.
    'u', // 16 Dark blue background, light blue square. MID ONLY.
    'v', // 21 Pink background, yellow castle. MID ONLY.
    'w', //  22 Dark blue background, dark blue cross in pink circle. MID ONLY.
];
