import React, { Component } from 'react'
import { AppConsumer, AppContext } from '../../context/AppContext'
import Cell from './Cell'
import Player from './Player'

export default class Field extends Component {
  getCell (x, y) {
    let cellIndex = y*5+x
    let piece = this.context.state.game.field.pieces[cellIndex]
    let position = {x: x, y: y}
    return <Cell position={position} piece={piece} key={cellIndex} />
  }

  render () {
    return (
      <AppConsumer>
        {({ state, mutations }) => (
          <div className='field-panel'>
            <Player playerIndex='1' />
            <div className='field'>
              <div className='field-rows'>
              {[4,3,2,1,0].map((y) => {
                return (
                  <div key={y} className='field-columns'>
                  {[0,1,2,3,4].map((x) => this.getCell(x, y))}
                  </div>
                )
              })}
              </div>
            </div>
            <Player playerIndex='0' />
          </div>
        )}
      </AppConsumer>
    )
  }
}

Field.contextType = AppContext
