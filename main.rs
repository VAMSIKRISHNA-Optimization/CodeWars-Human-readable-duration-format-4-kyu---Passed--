// const YEAR_DIVIDER: u64 = 31_557_600; // Julian
// const YEAR_DIVIDER: u64 = 31_556_952; // Gregorian
const YEAR_DIVIDER: u64 = 31_536_000; // Standard

fn format_duration(seconds: u64) -> String 
{
    if seconds == 0 { return "now".to_string(); } 
    
    // Core Logic 
    let mut duration = String::new();
    let mut all_counter : Vec<u64> = vec![0;5];
    match seconds
    {
        // Logic for seconds
        seconds if seconds < 60 =>
        {
            all_counter[4] = seconds;
        }
        
        // Logic for minutes and seconds
        seconds if seconds >= 60 && seconds < 3600 => 
        {
            let minutes_remaining = seconds/60;
            let seconds_remaning  = seconds%60;
            
            all_counter[3] = minutes_remaining; 
            all_counter[4] = seconds_remaning;
        }
        
        // Logic for hours, minutes and seconds
        seconds if seconds >= 3600 && seconds < 86400 => 
        {
            let hours_remaining   = seconds/3600;
            let minutes_remaining = (seconds%3600)/60;
            let seconds_remaining = (seconds%3600)%60; 
            
            all_counter[2] = hours_remaining;
            all_counter[3] = minutes_remaining; 
            all_counter[4] = seconds_remaining;
        }
        
        // Logic for days, hours, minutes and seconds
        seconds if seconds >= 86400 && seconds < 31556926 => 
        {
            let days_remaining    = seconds/86400;
            let hours_remaining   = (seconds%86400)/3600;
            let minutes_remaining = (((seconds%86400)%3600)/60);
            let seconds_remaining = (((seconds%86400)%3600)%60);
            
            all_counter[1] = days_remaining;
            all_counter[2] = hours_remaining;
            all_counter[3] = minutes_remaining; 
            all_counter[4] = seconds_remaining;
        }
        
        // Logic for years, days, hours, minutes and seconds
        seconds if seconds >= YEAR_DIVIDER => 
        {
            let years_remaining   = seconds/YEAR_DIVIDER;
            let new_seconds       = seconds%YEAR_DIVIDER;
            let days_remaining    = new_seconds/86400;
            let hours_remaining   = (new_seconds%86400)/3600;
            let minutes_remaining = (((new_seconds%86400)%3600)/60);
            let seconds_remaining = (((new_seconds%86400)%3600)%60);
            
            all_counter[0] = years_remaining;
            all_counter[1] = days_remaining;
            all_counter[2] = hours_remaining;
            all_counter[3] = minutes_remaining; 
            all_counter[4] = seconds_remaining;
        }
        _=>
        {
            return format!("Unknown");
        }
    }
    
    // Count non-zeros for comma seperation
    let mut non_zero_count = all_counter.clone().iter()
                             .filter(|&n| *n != 0)
                             .count();
    
    // String Formatting 
    // Years 
    if all_counter[0] == 1 
    {
        duration.push_str(&format!("{} year", all_counter[0]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    else if all_counter[0] > 1 
    {
        duration.push_str(&format!("{} years", all_counter[0]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    
    // Days 
    if all_counter[1] == 1 
    {
        duration.push_str(&format!("{} day", all_counter[1]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    else if all_counter[1] > 1 
    {
        duration.push_str(&format!("{} days", all_counter[1]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    
    // Hours 
    if all_counter[2] == 1 
    {
        duration.push_str(&format!("{} hour", all_counter[2]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    else if all_counter[2] > 1 
    {
        duration.push_str(&format!("{} hours", all_counter[2]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    
    // Minutes 
    if all_counter[3] == 1 
    {
        duration.push_str(&format!("{} minute", all_counter[3]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    else if all_counter[3] > 1 
    {
        duration.push_str(&format!("{} minutes", all_counter[3]));
        
        if non_zero_count > 2       { duration.push_str(", "); non_zero_count -=1; }
        else if non_zero_count == 2 { duration.push_str(" and "); non_zero_count -=1;}
    }
    
    
    // Seconds 
    if all_counter[4] == 1 
    {
        duration.push_str(&format!("{} second", all_counter[4]));
    }
    else if all_counter[4] > 1 
    {
        duration.push_str(&format!("{} seconds", all_counter[4]));
    }
    
    //println!("{:?}", all_counter);
    
    duration
    
}
