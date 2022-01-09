
mod reader;

fn main() {

    let start = "OOBFPNOPBHKCCVHOBCSO";
    let rules = reader::read("input.txt").unwrap();

    let mut polymer : [i64; 676] = [0; 676];

    println!("rules {:?}", rules.clone());

    for i_ch in 0..start.len()-1 {
        add_pair_to_polymer(&mut polymer, &start[i_ch..=i_ch+1]);
    }

    let num_iterations = 40;
    for i in 0..num_iterations {
        apply_rules_to_polymer(&mut polymer, &rules);
    }

    let result = compute_result(polymer);
    println!("result {}", result);

}

fn compute_result(polymer : [i64; 676]) -> i64
{
    let mut letters : [i64; 26] = [0;26];

    for i in 0..polymer.len() {
        letters[i % 26] += polymer[i];
        letters[i / 26] += polymer[i];
    }
    for letter in &mut letters {
        *letter = (*letter + 1) / 2;
    }

    println!("letters: {:?}", letters);

    let letters_above_zero : Vec<i64> = letters.into_iter().filter(|&x| x > 0).collect();
    return letters_above_zero.iter().max().unwrap() - letters_above_zero.iter().min().unwrap();
}

fn apply_rules_to_polymer(polymer : &mut [i64; 676],
                          rules : &Vec<[i64;3]>) 
{
    let mut polymer_copy : [i64; 676] = [0; 676];
    polymer_copy.clone_from_slice(&polymer[..]);

    for rule in rules 
    {
        let count = polymer[rule[0] as usize];
        polymer_copy[rule[0] as usize] -= count;
        polymer_copy[rule[1] as usize] += count;
        polymer_copy[rule[2] as usize] += count;
    }
    polymer.copy_from_slice(&polymer_copy[..]);
}

fn add_pair_to_polymer(polymer : &mut [i64; 676], pair : &str) 
{
    let letter_encoded : u32 =  26*(pair.chars().nth(0).unwrap() as u32 - 'A' as u32) + 
    pair.chars().nth(1).unwrap() as u32 - 'A' as u32;

    polymer[letter_encoded as usize] += 1;
}
