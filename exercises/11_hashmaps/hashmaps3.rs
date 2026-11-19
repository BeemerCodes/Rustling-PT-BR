// É dada uma lista de placares (um por linha) de uma partida de futebol. Cada
// linha tem o formato "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Exemplo: "England,France,4,2" (England marcou 4 gols, France 2).
//
// Você precisa construir uma tabela de placares contendo o nome do time, o
// número total de gols que o time marcou e o número total de gols que o time
// sofreu.

use std::collections::HashMap;

// Uma struct para armazenar os detalhes de gols de um time.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
    // O nome do time é a chave e sua struct associada é o valor.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTA: Usamos `unwrap` porque ainda não lidamos com tratamento de erros.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Preencha a tabela de placares com os detalhes extraídos.
        // Tenha em mente que os gols marcados pelo time 1 serão o número de gols
        // sofridos pelo time 2. Da mesma forma, os gols marcados pelo time 2 serão o
        // número de gols sofridos pelo time 1.
    }

    scores
}

fn main() {
    // Você pode experimentar aqui, se quiser.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_scores_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
