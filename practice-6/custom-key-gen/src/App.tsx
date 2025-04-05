import { useEffect, useState } from 'react'
import reactLogo from './assets/react.svg'
import solanaLogo from '/solana.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { Keypair } from '@solana/web3.js'

function App() {
  const [isGenerating, setIsGenerating] = useState(false)
  const [value, setValue] = useState('')
  const [results, setResults] = useState<KeypairResults | undefined>(undefined)

  const onInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setValue(event.target.value)
  }

  useEffect( () => {
    if (isGenerating) {
      const results = generateKeypair(value)
      setResults(results);
      setIsGenerating(false)
    }
  }
  , [isGenerating, value])

  return (
    <div className='flex flex-col gap-3'>
      <div className='flex justify-center'>
        <a href="https://solana.com/" target="_blank">
          <img src={solanaLogo} className="logo solana" alt="Vite logo" />
        </a>
        <a href="https://vite.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>{isGenerating ? 'Generating ...' : 'Generate your case insensitive keypair'}</h1>
      <StyledInput onChange={onInputChange} placeholder="Starts with ..." value={undefined} isDisabled={isGenerating} />
      <div className="card">
        <button disabled={!value || isGenerating} onClick={() => setIsGenerating(true)}>
          {`Generate public key that starts with '${value}'`}
        </button>
      </div>
      {results && <div>
        <h2>Public Key:</h2>
        <code>{JSON.stringify(results.publicKey, null, 2)}</code>
        <h2>Private Key:</h2>
        <code>{JSON.stringify(results.secretKey, null, 2)}</code>
        <h2>{`It took only ${results.iterationsCount} iterations`}</h2>
      </div>}
    </div>
  )
}

export default App

type StyledInputProps = {
  isDisabled: boolean;
  placeholder: string | undefined;
  value: string | undefined; 
  onChange: React.ChangeEventHandler<HTMLInputElement> | undefined;
}

function StyledInput({ placeholder, value, onChange, isDisabled }: StyledInputProps) {
  return (
    <input
      className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-300 leading-tight focus:outline-none focus:shadow-outline"
      disabled={isDisabled}
      type="text"
      placeholder={placeholder}
      value={value}
      maxLength={4}
      onChange={onChange}
    />
  );
}

interface KeypairResults {
  iterationsCount: number
  publicKey: string
  secretKey: number[]
}

export function generateKeypair(startsWith: string): KeypairResults {
  let keypair = Keypair.generate();
  let iterationsCount = 0
  const regex = new RegExp(`^${startsWith || ''}`, "i");
  while (!regex.test(keypair.publicKey.toBase58())) {
    keypair = Keypair.generate()
    iterationsCount++
  }
  return {
    iterationsCount,
    publicKey: keypair.publicKey.toBase58(),
    secretKey: Array.from(keypair.secretKey),
  }
}