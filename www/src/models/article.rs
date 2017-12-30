use controller::Msg;
use yew::html::Html;
use views::Viewable;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Article {
    pub title: String,
    pub publish_date: String, // todo change this boi.
    pub author: String, // This may point to a user in the future instead
    pub content: String, // TODO: Possibly make this HTML instead???
    pub id: String
}

impl Article {
    pub fn temp() -> Article {
        Article {
            title: "Joe Slays Again".to_string(),
            publish_date: "Today".to_string(),
            author: "Me".to_string(),
            content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur id turpis diam. Proin ullamcorper euismod lectus, ac molestie dui ultricies a. Aliquam gravida tellus id dui volutpat, id gravida libero iaculis. Sed ac dictum sem. Sed id elementum dui. Fusce efficitur vulputate placerat. Pellentesque leo leo, tristique vitae velit quis, rutrum porttitor metus. Duis dignissim, turpis eget tempus hendrerit, risus tellus maximus libero, eu convallis tortor risus id felis.

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc sit amet lorem sed nunc fermentum semper a sit amet lorem. Vestibulum tellus eros, rhoncus in ligula ut, mollis feugiat metus. Sed non dignissim risus. Quisque ac quam a mauris viverra maximus placerat non lacus. Aenean vitae eleifend orci. Donec erat tortor, ultrices non commodo eu, viverra in odio. In rutrum id velit ut porttitor. Praesent vel massa sapien.

Sed diam sapien, malesuada ut semper id, posuere sed eros. Etiam viverra erat sit amet risus rutrum, vel placerat tortor elementum. Pellentesque cursus finibus nisl ac imperdiet. Mauris posuere elit erat, at molestie magna porta ac. Pellentesque et ligula a erat euismod placerat ac sit amet ipsum. Sed luctus condimentum ante. Integer varius mattis ligula. Aliquam erat volutpat. Proin porttitor aliquam massa non malesuada. Nam dignissim sed nibh id ultrices. Quisque facilisis velit nec erat scelerisque viverra. Ut ac volutpat ex. Praesent et ex vitae erat rhoncus consequat. Sed eu tempus est.

Ut laoreet quam sit amet sapien posuere, at semper leo malesuada. Proin semper, dui in sagittis sagittis, augue nisi convallis lorem, ut feugiat libero augue et lorem. Duis varius at ex at fermentum. Integer ultricies lobortis erat, sed rhoncus mi posuere sed. Vestibulum at commodo elit. Sed imperdiet ornare justo ac hendrerit. Sed egestas diam eu nunc fringilla, a rutrum quam dictum. Duis laoreet turpis ut libero porta venenatis. Cras ac semper orci. Aenean laoreet, ante sit amet accumsan elementum, libero sapien aliquam felis, ut pretium est ligula vitae nisi. Quisque vel commodo neque, ac ullamcorper mauris.

Fusce vel lacinia sapien, nec condimentum enim. Suspendisse fermentum neque quis quam tempor viverra. Cras commodo, felis nec sollicitudin mollis, eros metus imperdiet lacus, eget lobortis neque tellus in eros. Vestibulum vestibulum enim non ex tincidunt eleifend. Donec dictum efficitur risus, non aliquam felis lobortis a. In bibendum lorem sit amet tellus pretium, eget pulvinar eros vehicula. Maecenas pulvinar ligula sapien, sit amet egestas dui lacinia sit amet. Quisque purus ligula, malesuada nec ipsum in, tempor malesuada urna. Donec id vulputate ex, vitae interdum justo. Mauris dapibus quam tellus, vitae volutpat lorem porttitor cursus. Maecenas vitae sollicitudin est, id euismod nisi. Praesent cursus ultrices elementum. ".to_string(),
            id: "hello".to_string(),
        }
    }
}

impl Viewable<Msg> for Article {
    fn view(&self) -> Html<Msg> {

        html!{
            <div>
                <h2>
                    { self.title.clone() }
                </h2>

                <h6>
                    { format!("By: {}", self.author)}
                </h6>
                <h6>
                    { format!("Published: {}", self.publish_date)}
                </h6>

                <div>
                    { self.content.clone() }
                </div>

            </div>
        }
    }
}