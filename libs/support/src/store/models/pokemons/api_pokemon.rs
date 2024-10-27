use serde::Serialize;
use serde::Deserialize;
use error::Error;

//***** Pokemon *******/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiPokemon {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub base_experience: Option<i32>,
    pub height: Option<i32>,
    pub is_default: Option<bool>,
    pub order: Option<i32>,
    pub weight: Option<i32>,
    pub sprites: Option<ApiSprites>,
    pub abilities: Vec<ApiAbilities>,
    pub stats: Vec<ApiStats>,
    pub types: Vec<ApiTypes>,
}

impl ApiPokemon {
    pub async fn get_pokemons_from_api(starting_index: &i32, amount: &i32) -> Result<Vec<ApiPokemon>, Error> {
        let response = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/?offset={}&limit={}", starting_index-1, amount))
            .await?;
        let pokemon_results: PokemonResults = response.json().await?;

        let mut api_pokemons = Vec::new();
        for pokemon_result in pokemon_results.results {
            if let Some(url) = pokemon_result.url {
                api_pokemons.push(reqwest::get(&url).await?.json().await?);
            }
        }

        Ok(api_pokemons)
    }

    pub async fn get_specific_pokemon_from_api(index: i32) -> Result<ApiPokemon, Error> {
        let response = reqwest::get(&format!("https://pokeapi.co/api/v2/pokemon/{}", index)).await?;
        let specific_pokemon: ApiPokemon = response.json().await?;
    
        Ok(specific_pokemon)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonResult {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonResults {
    pub results: Vec<PokemonResult>,
}

//***** Sprites (Pokemon Image) *******/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiSprites {
    pub front_default: Option<String>,
}

//***** Abilities *******/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiAbilities{
    pub ability: Option<ApiAbility>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiAbility{
    pub name: Option<String>,
}

//***** Stats *******/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiStats{
    pub base_stat: Option<i32>,
    pub effort: Option<i32>,
    pub stat: Option<ApiStat>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiStat{
    pub name: Option<String>,
}

//***** Types *******/
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiTypes{
    #[serde(rename = "type")]
    pub type_: Option<ApiType>,
    pub slot: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiType{
    pub name: Option<String>,
}