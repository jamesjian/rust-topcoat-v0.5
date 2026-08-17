use topcoat::{
	Result,
	view::{component, view},

};

#[component]
async fn hello(name: &str)->Result{
	view! {<h1> "Hello, " (name) "!"</h1>}
}


#[component]
pub async fn homepage()->Result{
	view! {
		<!DOCTYPE html>
		<html>
			<head>
				<title>"rust"</title>
			</head>
			<body>
				<h2>hello(name:"My Name")</h2>
			</body>
		</html>
	}
}
