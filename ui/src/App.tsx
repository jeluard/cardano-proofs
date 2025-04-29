import './App.css'
import latestProofs from '../../assets/proofs/latest.json' with {type: 'json'};

function App() {
  return (
    <>
      <h1>Blocks</h1>
      <table>
      {Object.entries(latestProofs).map(([blockNumber, {proofs}]) =>
        <tr>
          <td>{blockNumber}</td>
          <td>
            {Object.entries(proofs).map(([prover, proof]) => (
              <>
                <span onClick={() => alert(proof)}>{prover}</span>
              </>
            ))}
          </td>
        </tr>
      )}
    </table>
    </>
  )
}

export default App
