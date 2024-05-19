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

// types from backend, just copied over to typescript, should find a better way to share them

// pub struct AddRequest {
// 	pub name: Option<String>,
// 		pub url: String,
// 		pub custom: Option<String>,
// }

export interface OkApiResponse {}

export type ApiResponse = OkApiResponse | ApiError;

export interface StringApiResponse extends OkApiResponse {
	string: string;
}

export interface AddRequest extends OkApiResponse {
	name?: string;
	url: string;
	custom?: string;
}

//pub struct AddResponse {
//         pub shortened: String,
//     }

export interface AddResponse extends OkApiResponse {
	shortened: string;
}

// pub struct DeleteRequest {
// 	pub shortened: String,
// }

export interface DeleteRequest extends OkApiResponse {
	shortened: string;
}

// Inforesponse is just InfoLinkView
// #[derive(Serialize)]
// pub struct InfoLinkView {
//     pub name: String,
//     pub original: String,
//     pub clicks: Vec<InfoClick>,
//     pub created_at: DateTime,
// }
//
// #[derive(Serialize)]
// pub struct InfoClick {
//     pub clicked_at: String,
//     pub address: String,
//     pub user_agent: Option<String>,
// }

export interface InfoLinkView extends OkApiResponse {
	name: string;
	original: string;
	shortened: string;
	clicks: InfoClick[];
	created_at: string;
}

export interface InfoClick {
	clicked_at: Date;
	address: string;
	user_agent?: string;
}

// pub struct ListResponse {
// 	links: Vec<Link>,
// }

export interface ListResponse extends OkApiResponse {
	links: Link[];
}

// pub struct Link {
// 	name: String,
// 		original: String,
// 		shortened: String,
// 		created_at: DateTime,
// }

export interface Link {
	name: string;
	image?: string;
	original: string;
	shortened: string;
	created_at: string;
}

export interface ApiError extends OkApiResponse {
	error: string;
	description: string;
}
