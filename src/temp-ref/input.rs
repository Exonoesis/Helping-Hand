//pub fn animate_entity(
//    mut entity_query: Query<(&mut TextureAtlas, &DirectionFacing), Changed<DirectionFacing>>,
//) {
//    if entity_query.is_empty() {
//        return;
//    }
//
//    for (mut sprite, facing) in entity_query.iter_mut() {
//        match facing {
//            DirectionFacing::Up => {
//                sprite.index = 0;
//            }
//            DirectionFacing::Down => {
//                sprite.index = 1;
//            }
//            DirectionFacing::Left => {
//                sprite.index = 2;
//            }
//            DirectionFacing::Right => {
//                sprite.index = 3;
//            }
//        }
//    }
//}
