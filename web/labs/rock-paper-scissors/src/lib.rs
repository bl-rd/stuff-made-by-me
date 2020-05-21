use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Element, HtmlElement};
use js_sys::{Math};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Choice {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Win,
    Lose,
    Draw
}

enum State {
    Menu,
    Play,
    Win,
    Lose,
    Draw
}

enum ButtonState {
    Disabled,
    Enabled
}

// The button selectors
const BUTTON_ROCK_SELECTOR: &str = "#rock";
const BUTTON_PAPER_SELECTOR: &str = "#paper";
const BUTTON_SCISSORS_SELECTOR: &str = "#scissors";
const BUTTON_ACTION_SELECTOR: &str = "#again";
const BUTTON_PLAY_SELECTOR: &str = "#play";

// the game UI selectors
const UI_OUTCOME_SELECTOR: &str = ".ui__outcome";
const UI_PLAY_SELECTOR: &str = ".ui__play";
const UI_MENU_SELECTOR: &str = ".ui__menu";

const IMAGE_PLAYER_SELECTOR: &str = ".game__element--player .game__element__image";
const IMAGE_PLAYER_SELECTOR_2: &str = ".game__element--player img";
const IMAGE_AI_SELECTOR: &str = ".game__element--ai .game__element__image";
const IMAGE_AI_SELECTOR_2: &str = ".game__element--ai img";
const IMAGE_SELECTOR: &str = ".game__element__image";
const TEXT_PLAYER_SELECTOR: &str = ".game__element--player .game__element__text";
const TEXT_AI_SELECTOR: &str = ".game__element--ai .game__element__text";
const TEXT_SELECTOR: &str = ".game__element__text";
const RESULT_SELECTOR: &str = ".ui__result";

// choice variables
const CHOICE_ROCK: &str = "✊";
const CHOICE_PAPER: &str = "🤚";
const CHOICE_SCISSORS: &str = "✌";

// result variables
const RESULT_WIN: &str = "👍";
const RESULT_LOSE: &str = "👎";
const RESULT_DRAW: &str = "🖐";

// other options
const STATE_TRANSITION_TIME: i32 = 2000;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // get references to the UI elements
    let rock_button = query_selector(BUTTON_ROCK_SELECTOR).unwrap();
    let paper_button = query_selector(BUTTON_PAPER_SELECTOR).unwrap();
    let scissors_button = query_selector(BUTTON_SCISSORS_SELECTOR).unwrap();
    let again_button = query_selector(BUTTON_ACTION_SELECTOR).unwrap();
    let player_image = query_selector(IMAGE_PLAYER_SELECTOR_2).unwrap();
    let ai_image = query_selector(IMAGE_AI_SELECTOR_2).unwrap();

    // all the button callbacks
    let rock_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Rock);
    }) as Box<dyn FnMut(_)>);

    let paper_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Paper);
    }) as Box<dyn FnMut(_)>);

    let scissors_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Scissors);
    }) as Box<dyn FnMut(_)>);

    let again_button_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        hide_elements(TEXT_SELECTOR);
        hide_elements(IMAGE_SELECTOR);
        hide_elements(UI_OUTCOME_SELECTOR);
        hide_elements(RESULT_SELECTOR);
        update_image(&player_image, &Choice::Rock);
        update_image(&ai_image, &Choice::Rock);
        change_state(State::Play);
    }) as Box<dyn FnMut(_)>);

    // Add the event listeners to all the game buttons
    rock_button.add_event_listener_with_callback("click", rock_closure.as_ref().unchecked_ref()).unwrap();
    rock_closure.forget();
    paper_button.add_event_listener_with_callback("click", paper_closure.as_ref().unchecked_ref()).unwrap();
    paper_closure.forget();
    scissors_button.add_event_listener_with_callback("click", scissors_closure.as_ref().unchecked_ref()).unwrap();
    scissors_closure.forget();

    again_button.add_event_listener_with_callback("click", again_button_closure.as_ref().unchecked_ref()).unwrap();
    again_button_closure.forget();

    // start the game!
    change_state(State::Menu);

    Ok(())
}

/// Convert the game choice into it's corresponding String value
fn get_choice(choice: &Choice) -> String {
    match choice {
        Choice::Rock => String::from("Rock"),
        Choice::Paper => String::from("Paper"),
        Choice::Scissors => String::from("Scissors")
    }
}

/// Get the image based on the choice
fn get_choice_image(choice: &Choice) -> String {
    match choice {
        Choice::Rock => String::from(CHOICE_ROCK),
        Choice::Paper => String::from(CHOICE_PAPER),
        Choice::Scissors => String::from(CHOICE_SCISSORS)
    }
}

/// Convert the outcome to it's corresponding String value
fn get_outcome(outcome: &Outcome) -> String {
    match outcome {
        Outcome::Win => String::from("You win!"),
        Outcome::Lose => String::from("You lose!"),
        Outcome::Draw => String::from("It's a draw!")
    }
}

/// Get a random game option
fn get_random_choice() -> Option<Choice> {
    let num = get_random_int(3);
    match num {
        0 => Some(Choice::Rock),
        1 => Some(Choice::Paper),
        2 => Some(Choice::Scissors),
        _ => None
    }
}

/// Determine the outcome based on the two choices
fn battle(player_choice: &Choice, ai_choice: &Choice) -> Outcome {
    match player_choice {
        Choice::Rock => {
            match ai_choice {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Lose,
                Choice::Scissors => Outcome::Win
            }
        },
        Choice::Paper => {
            match ai_choice {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Draw,
                Choice::Scissors => Outcome::Lose
            }
        },
        Choice::Scissors => {
            match ai_choice {
                Choice::Rock => Outcome::Lose,
                Choice::Paper => Outcome::Win,
                Choice::Scissors => Outcome::Draw
            }
        }
    }
}

/// Helper function to enable/disable the play buttons
fn update_button_disabled_state(state: ButtonState) {
    let rock_button = query_selector(BUTTON_ROCK_SELECTOR).unwrap();
    let paper_button = query_selector(BUTTON_PAPER_SELECTOR).unwrap();
    let scissors_button = query_selector(BUTTON_SCISSORS_SELECTOR).unwrap();

    match state {
        ButtonState::Disabled => {
            rock_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
            paper_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
            scissors_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
        },
        ButtonState::Enabled => {
            rock_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
            paper_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
            scissors_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
        }
    }    
}

/// Function to have a simple game, based on player choice
fn play(player_choice: Choice) {
    update_button_disabled_state(ButtonState::Disabled);

    let window = web_sys::window().expect("Should have a window...");
    let closure = Closure::wrap(Box::new(move || {
        console_log(format!("You chose {}", get_choice(&player_choice)).as_str());
        let ai_choice = get_random_choice().unwrap();
        console_log(format!("Opponent chose {}", get_choice(&ai_choice)).as_str());

        let player_image = query_selector(IMAGE_PLAYER_SELECTOR).unwrap();
        let player_image_2 = query_selector(IMAGE_PLAYER_SELECTOR_2).unwrap();
        let player_text = query_selector(TEXT_PLAYER_SELECTOR).unwrap();
        let ai_image = query_selector(IMAGE_AI_SELECTOR).unwrap();
        let ai_image_2 = query_selector(IMAGE_AI_SELECTOR_2).unwrap();
        let ai_text = query_selector(TEXT_AI_SELECTOR).unwrap();

        // update the images
        player_image.set_inner_text(get_choice_image(&player_choice).as_str());
        ai_image.set_inner_text(get_choice_image(&ai_choice).as_str());

        update_image(&player_image_2, &player_choice);
        update_image(&ai_image_2, &ai_choice);

        // update the text
        player_text.set_inner_text(get_choice(&player_choice).as_str());
        ai_text.set_inner_text(get_choice(&ai_choice).as_str());
        show_elements(TEXT_SELECTOR);

        let outcome = battle(&player_choice, &ai_choice);
        match outcome {
            Outcome::Draw => change_state(State::Draw),
            Outcome::Win => change_state(State::Win),
            Outcome::Lose => change_state(State::Lose)
        }
    }) as Box<dyn FnMut()>);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), STATE_TRANSITION_TIME)
        .expect("Unable to do timeout");
    closure.forget();

    console_log("Waiting 3 seconds for transition...");
}

/// Get a random u64 up to, but not including, the max
fn get_random_int(max: u64) -> u64 {
    use Math::{floor, random};
    floor(random() * floor(max as f64)) as u64
}

/// Wrapper for the query_element API
fn query_selector(selector: &str) -> Option<HtmlElement> {
    let body: Element = web_sys::window()
        ?.document()
        ?.body()
        ?.into();

    let element = body.query_selector(selector)
        .ok()?
        .unwrap();
    
    // convert into HtmlElement as this has more useful methods
    element.dyn_into::<HtmlElement>().ok()
}

fn query_selector_all(selector: &str) -> Option<Vec::<HtmlElement>> {
    let body: web_sys::Document = web_sys::window()
        ?.document()
        ?.into();
    
    let nodes = body.query_selector_all(selector)
        .ok()?;
    
    let mut elements: Vec::<HtmlElement> = Vec::<HtmlElement>::new();

    for n in 0..nodes.length() {
        let e: HtmlElement = nodes
            .item(n)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .ok()
            .unwrap();
        elements.push(e);
    };

    Some(elements)
}

/// wrapper for the web_sys console::log_1 method
fn console_log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// function for handling the changes in the game state
fn change_state(state: State) {
    match state {
        State::Menu => init_menu_state(),
        State::Play => init_play_state(),
        State::Win => init_outcome_state(Outcome::Win),
        State::Lose => init_outcome_state(Outcome::Lose),
        State::Draw => init_outcome_state(Outcome::Draw)
    }
}

/// logic for the "menu" state of the game
fn init_menu_state() {

    show_elements(UI_MENU_SELECTOR);

    // get the button element
    let play_button = query_selector(BUTTON_PLAY_SELECTOR).unwrap();

    // if the button is clicked, update the state
    let play_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        hide_elements(UI_MENU_SELECTOR);
        change_state(State::Play);
    }) as Box<dyn FnMut(_)>);
    play_button.add_event_listener_with_callback("click", play_closure.as_ref().unchecked_ref()).unwrap();
    play_closure.forget();
}

fn init_play_state() {

    show_elements(UI_PLAY_SELECTOR);

    update_button_disabled_state(ButtonState::Enabled);
}

fn init_outcome_state(outcome: Outcome) {

    hide_elements(UI_PLAY_SELECTOR);
    
    show_elements(IMAGE_SELECTOR);

    let player_image = query_selector(IMAGE_PLAYER_SELECTOR).unwrap();
    let ai_image = query_selector(IMAGE_AI_SELECTOR).unwrap();

    let result_text = query_selector(format!("{} h2", RESULT_SELECTOR).as_str()).unwrap();
    result_text.set_inner_text(get_outcome(&outcome).as_str());

    // closure for the set_timeout.
    let closure = Closure::wrap(Box::new(move || {

        show_elements(UI_OUTCOME_SELECTOR);
        show_elements(RESULT_SELECTOR);

        match outcome {
            Outcome::Draw => {
                player_image.set_inner_text(RESULT_DRAW);
                ai_image.set_inner_text(RESULT_DRAW);
            },
            Outcome::Lose => {
                player_image.set_inner_text(RESULT_LOSE);
                ai_image.set_inner_text(RESULT_WIN);
            },
            Outcome::Win => {
                player_image.set_inner_text(RESULT_WIN);
                ai_image.set_inner_text(RESULT_LOSE);
            }
        };

    }) as Box<dyn FnMut()>);

    let window = web_sys::window().expect("Should have a window...");
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            STATE_TRANSITION_TIME
        )
        .unwrap();
    closure.forget();
}

/// hide elements
fn hide_elements(selector: &str) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        elem
            .set_attribute("hidden", "true")
            .expect(format!("cannot hide element '{}'", selector).as_str());        
    }
}

/// show elements
fn show_elements(selector: &str) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        elem
            .remove_attribute("hidden")
            .expect(format!("Unable to remove hidden attribute from {}", selector).as_str());
    }
}

/// Update an image based on user/ai choice
fn update_image(img: &HtmlElement, selection: &Choice) {
    let img_src: &str;
    let img_alt: &str;

    match selection {
        Choice::Rock => {
            img_src = "rock-500.png";
            img_alt =  "A hand doing a rock sign"; 
        },
        Choice::Paper => {
            img_src = "paper-500.png";
            img_alt =  "A hand doing a paper sign"; 
        },
        Choice::Scissors => {
            img_src = "scissors-500.png";
            img_alt =  "A hand doing a scissors sign"; 
        },
    };

    img.set_attribute("src", img_src).expect("unable to set img src");
    img.set_attribute("alt", img_alt).expect("unable to set img alt");
}