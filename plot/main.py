import pandas as pd
import plotly.graph_objects as go

df = pd.read_csv("./data/mobius_points.csv")

fig = go.Figure(
    data=[
        go.Scatter3d(
            x=df["x"],
            y=df["y"],
            z=df["z"],
            mode="markers",
            marker=dict(size=1, color=df["z"], colorscale="Viridis"),
        )
    ]
)

fig.update_layout(
    scene=dict(xaxis_visible=False, yaxis_visible=False, zaxis_visible=False),
    margin=dict(r=0, l=0, b=0, t=0),
    paper_bgcolor="white",
    scene_aspectmode="data",
)

fig.show()
