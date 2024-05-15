// #[derive(Serialize)]
// pub struct InfoLinkView {
// 	pub name: String,
// 		pub original: String,
// 		pub clicks: Vec<InfoClick>,
// 		pub created_at: DateTime,
// }
//
// #[derive(Serialize)]
// pub struct InfoClick {
// 	pub clicked_at: String,
// 		pub address: String,
// 		pub user_agent: Option<String>,
// }
//

export interface InfoLinkView {
	name: string;
	original: string;
	clicks: InfoClick[];
	created_at: string;
}

export interface InfoClick {
	clicked_at: string;
	address: string;
	user_agent?: string;
}
