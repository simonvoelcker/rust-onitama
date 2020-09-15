import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Cell from './Cell'

export default class Field extends Component {
  constructor (props) {
    super(props)
    this.state = {
      pieces: [
        [null, null, null, null, 'A'],
        [null, null, null, null, null],
        [null, null, null, null, null],
        [null, null, null, null, null],
        [null, null, null, 'A', null],
      ],
    }
  }

  render () {
    let {pieces} = this.state
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='field-panel'>
            <div className='field-rows'>
            {pieces.map((row, rowIndex) => {
              return (
                <div className='field-columns'>
                {row.map((cell, cellIndex) => <Cell content={cell} /> )}
                </div>
              )
            })}
            </div>
          </div>
        )}
      </AppConsumer>
    )
  }
}

Field.contextType = AppContext
