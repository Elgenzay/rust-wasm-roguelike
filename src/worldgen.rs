pub mod worldgen {

	pub mod room {
		use rand::Rng;

		/// Percent of room length_width_sum to deviate from 50 when rolling a
		/// random number to determine room dimensions.
		///
		/// At 0, all rooms are square.
		///
		/// At 50, rooms can be a single unit tall or wide.
		const ROOM_SQUARE_DEVIATION_THRESHOLD: i8 = 20;

		pub enum RoomSize {
			LARGE,
			MEDIUM,
			SMALL,
			CUSTOM(i32),
		}

		#[derive(Debug)]
		pub struct Room {
			pub width: i32,
			pub height: i32,
		}

		impl Room {
			pub fn new(size: RoomSize) -> Room {
				let length_width_sum: i32 = match size {
					RoomSize::SMALL => 8,
					RoomSize::MEDIUM => 16,
					RoomSize::LARGE => 32,
					RoomSize::CUSTOM(a) => a,
				};
				let deviation: i32 = (length_width_sum as f32
					* (ROOM_SQUARE_DEVIATION_THRESHOLD as f32 * 0.01)) as i32;
				if deviation == 0 {
					return Room {
						width: length_width_sum / 2,
						height: length_width_sum / 2,
					};
				}
				let min = (length_width_sum / 2) - deviation;
				let max = (length_width_sum / 2) + deviation;
				let width = rand::thread_rng().gen_range(min..max);
				let height = length_width_sum - width;
				if rand::random() {
					return Room { width, height };
				}
				Room {
					width: height,
					height: width,
				}
			}
		}
	}

	struct Area {}
}
