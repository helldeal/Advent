use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Ouvrir le fichier
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    // Parcourir chaque ligne du fichier
    for line in reader.lines() {
        let line = line?;
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        
        // Parcourir chaque caractère de la ligne
        for c in line.chars() {
            // Vérifier si le caractère est un chiffre
            if let Some(digit) = c.to_digit(10) {
                // Si c'est le premier chiffre trouvé dans la ligne, le sauvegarder
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                // Toujours mettre à jour la valeur du dernier chiffre trouvé
                last_digit = Some(digit);
            }
        }
        
        // Si un premier et un dernier chiffre ont été trouvés, calculer la valeur de calibration
        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            let calibration_value = first * 10 + last;
            sum+=calibration_value;
        }
    }
    println!("Somme de calibration: {}", sum);
    Ok(())
}
