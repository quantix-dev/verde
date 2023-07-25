local Fusion = require(script.Parent.Parent.Parent.packages.fusion)
local Types = require(script.Parent.Parent.Parent.types)

local New = Fusion.New
local Children = Fusion.Children

return function()
	return New("TextButton") {

		[Children] = {
			New("UICorner") {},
		},
	}
end :: Types.Component<{}>
