import './App.css'
import latestProofs from '../../assets/proofs/latest.json' with {type: 'json'};

function App() {
  return (
    <>
      <h1>Blocks</h1>
      <div>{Object.keys(latestProofs)}</div>
    </>
  )
}

export default App
