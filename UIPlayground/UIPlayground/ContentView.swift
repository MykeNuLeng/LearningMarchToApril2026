import SwiftUI

struct ContentView: View {
	@State private var isSheetVisable = true
	@State private var isAnimating = false

    var body: some View {
		NavigationStack {
			Text("Hello World!")
				.toolbar {
					Button {
						isSheetVisable = true
					} label: {
						Image(systemName: "trash")
					}
					.foregroundStyle(.red)
				}
				.sheet(isPresented: $isSheetVisable) {
					SheetContentView(isVisable: $isSheetVisable)
				}
		}
    }
}

struct SheetContentView: View {
	@Binding var isVisable: Bool
	@State private var isExploding = false
	@State private var isAnimating: Bool = false

	var body: some View {
		ZStack {
			LinearGradient(colors: [.red, .orange, .yellow], startPoint: .topLeading, endPoint: .bottomTrailing)
				.ignoresSafeArea()
			VStack {
				Spacer()

				Text("Do you want to delete the data in database?")
					.multilineTextAlignment(.center)
					.fixedSize(horizontal: false, vertical: true)
					.font(.largeTitle.bold())
					.padding(30)
					.foregroundStyle(.black)
					.shadow(radius: 50)
					.opacity(0.9)

				Spacer()
				Spacer()
				Spacer()

				ZStack {
					if isExploding {
						Circle()
							.padding(50)
							.opacity(0)
						ForEach(0..<60) { i in
							ExplodingFragment(index: i)
						}
					} else {
						Circle()
							.fill(.red.opacity(isAnimating ? 0.0 : 0.8))
							.scaleEffect(isAnimating ? 0.8 : 0.1)
							.padding(50)
							.animation(
								.easeInOut(duration: 1.5).repeatForever(autoreverses: false),
								value: isAnimating
							)
							.onAppear {
								isAnimating = true
							}
							.onDisappear { isAnimating = false }

						Button {
							isAnimating = false
							isExploding = true
						} label: {
							Text("💣")
								.font(.system(size: 100))
						}
					}
				}

				Spacer()
				Spacer()
				Spacer()
			}
		}
		.onChange(of: isExploding) { _, exploding in
			if exploding {
				DispatchQueue.main.asyncAfter(deadline: .now() + 0.8) {
					isVisable = false
				}
			}
		}
	}
}



struct ExplodingFragment: View {
	let index: Int
	@State private var offset: CGSize = .zero
	@State private var opacity: Double = 1
	@State private var rotation: Double = 0
	let colour: Color = [.red, .orange, .yellow].randomElement()!

	var body: some View {
		Rectangle()
			.fill(colour.opacity(opacity))
			.frame(width: 10, height: 15)
			.rotationEffect(.radians(rotation))
			.offset(offset)
			.onAppear {
				let angle = Double(index) * 2 * .pi / 60
				let distance: Double = Double.random(in: 50..<250)
				withAnimation(.easeOut(duration: 0.8)) {
					offset = CGSize(
						width: cos(angle) * distance,
						height: sin(angle) * distance
					)
					opacity = 0.3
					rotation = Double.random(in: -3 * .pi ... 3 * .pi)
				}
			}
	}
}

#Preview {
    ContentView()
}
